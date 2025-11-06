use axum::{routing::get, Router};
use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::{header, Client};
use serde_json::Value;
use std::{collections::HashSet, fs};
use tokio::time::{sleep, Duration};

const GITHUB_API_BASE: &str = "https://api.github.com";

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let repos_file = fs::read_to_string("data/repos.json").expect("Failed to read repos.json");
    let repos: Vec<String> = serde_json::from_str(&repos_file).expect("Invalid JSON in repos.json");

    let client = Client::new();

    // Run repository statistics collection in parallel
    let mut tasks = FuturesUnordered::new();
    for repo in repos {
        let client_clone = client.clone();
        tasks.push(tokio::spawn(async move {
            get_statistics(&client_clone, &repo).await
        }));
    }

    let mut all_users = HashSet::new();
    let mut all_contributors = HashSet::new();

    while let Some(result) = tasks.next().await {
        if let Ok((users, contributors)) = result {
            all_users.extend(users);
            all_contributors.extend(contributors);
        }
    }

    println!("Total FIWARE users: {}", all_users.len());
    println!("Total FIWARE developers: {}", all_contributors.len());

    // Simple Axum web server (optional)
    // let app = Router::new().route("/", get(|| async { "GitHub Stats Collector Running 🚀" }));
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}

async fn get_statistics(client: &Client, repo: &str) -> (HashSet<String>, HashSet<String>) {
    println!("Fetching stats for {repo}...");

    let per_page = 100;
    let params = vec![("per_page", per_page.to_string()), ("page", "1".to_string())];

    // --- Stargazers ---
    let url = format!("{}/repos/{}/stargazers", GITHUB_API_BASE, repo);
    let stargazers = get_data(client, &url, &params).await;
    let stargazers_logins: HashSet<String> =
        stargazers.iter().filter_map(|x| x["login"].as_str().map(String::from)).collect();

    // --- Contributors (repo + forks in parallel) ---
    let url = format!("{}/repos/{}/contributors", GITHUB_API_BASE, repo);
    let contributors_repo = get_data(client, &url, &params).await;

    let url = format!("{}/repos/{}/forks", GITHUB_API_BASE, repo);
    let forks = get_data(client, &url, &params).await;

    let mut tasks = FuturesUnordered::new();
    for fork in forks {
        if let Some(contrib_url) = fork["url"].as_str() {
            let fork_contrib_url = format!("{}/contributors", contrib_url);
            let client_clone = client.clone();
            let params_clone = params.clone();
            tasks.push(tokio::spawn(async move {
                get_data(&client_clone, &fork_contrib_url, &params_clone).await
            }));
        }
    }

    let mut all_contributors = contributors_repo;
    while let Some(result) = tasks.next().await {
        if let Ok(data) = result {
            all_contributors.extend(data);
        }
    }

    let contributors_logins: HashSet<String> =
        all_contributors.iter().filter_map(|x| x["login"].as_str().map(String::from)).collect();

    // --- Watchers ---
    let url = format!("{}/repos/{}/subscribers", GITHUB_API_BASE, repo);
    let subscribers = get_data(client, &url, &params).await;
    let subscribers_logins: HashSet<String> =
        subscribers.iter().filter_map(|x| x["login"].as_str().map(String::from)).collect();

    // --- Issues ---
    let mut issue_params = params.clone();
    issue_params.push(("state", "all".to_string()));
    let url = format!("{}/repos/{}/issues", GITHUB_API_BASE, repo);
    let issues = get_data(client, &url, &issue_params).await;
    let issuers_logins: HashSet<String> = issues
        .iter()
        .filter_map(|x| x["user"]["login"].as_str().map(String::from))
        .collect();

    // --- Combine ---
    let mut total_users = stargazers_logins.clone();
    total_users.extend(contributors_logins.clone());
    total_users.extend(subscribers_logins);
    total_users.extend(issuers_logins);

    println!(
        "[{}] Stargazers: {}, Developers: {}, Total Users: {}",
        repo,
        stargazers_logins.len(),
        contributors_logins.len(),
        total_users.len()
    );

    (total_users, contributors_logins)
}

async fn get_data(client: &Client, url: &str, params: &Vec<(&str, String)>) -> Vec<Value> {
    let github_token = std::env::var("GITHUB_TOKEN").expect("Missing GITHUB_TOKEN env var");

    let mut all_data = Vec::new();
    let mut page: usize = 1;

    loop {
        let mut query: Vec<(&str, String)> = params.clone();
        query.push(("page", page.to_string()));

        let res = client
            .get(url)
            .header(header::ACCEPT, "application/vnd.github.v3+json")
            .header(header::USER_AGENT, "axum-github-client")
            .bearer_auth(&github_token)
            .query(&query)
            .send()
            .await
            .unwrap();

        if res.status() == 403 {
            println!("Rate limit reached, waiting 1 hour...");
            sleep(Duration::from_secs(3610)).await;
            continue;
        }

        if !res.status().is_success() {
            let err = res.text().await.unwrap_or_default();
            eprintln!("Error fetching {}: {}", url, err);
            break;
        }

        let headers = res.headers().clone(); // Clone headers before consuming res
        let data: Vec<Value> = match res.json().await {
            Ok(v) => v,
            Err(_) => break,
        };

        if data.is_empty() {
            break;
        }

        all_data.extend(data);

        let has_next = headers
            .get("link")
            .and_then(|v| v.to_str().ok())
            .map(|link| link.contains("rel=\"next\""))
            .unwrap_or(false);

        if !has_next {
            break;
        }

        page += 1;
    }

    all_data
}
