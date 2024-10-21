use std::collections::HashMap;

use reqwest::header;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}


// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let url= "https://api.github.com/repos/telefonicaid/fiware-orion/stargazers";
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static("Bearer <YOUR TOKEN HERE>"));
    headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/vnd.github.v3+json"));
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("reqwest"));

    // Set up the query parameters for the request
    let mut params = HashMap::new();
    params.insert("per_page", 100);
    params.insert("page", 1);


    let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;

    println!("body = {body:?}");

    let response = client
    .get(url)
    .headers(headers)
    .query(&params)
    .send()
    .await?
    .text()
    .await;

    // Handle the response
    println!("{:#?}", response);

    //let users: Vec<User> = response.json().await;
    //println!("{:?}", users);
    Ok(())
}

