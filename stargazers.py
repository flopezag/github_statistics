import requests
import json
import time


def get_statistics(repo):
    page = 1
    per_page = 100

    url = 'https://api.github.com/repos/{}/stargazers'.format(repo)
    params = {'per_page': per_page, 'page': page}
    stargazers = get_data(url=url, params=params)
    stargazers = [x['login'] for x in stargazers]
    print(f"Total stargazers: {len(stargazers)}")

    url = 'https://api.github.com/repos/{}/contributors'.format(repo)
    params = {'per_page': per_page, 'page': page}
    contributors_repo = get_data(url=url, params=params)
    url = 'https://api.github.com/repos/{}/forks'.format(repo)
    forks = get_data(url=url, params=params)
    forks_url = [x['url']+'/contributors' for x in forks]
    for ix in range(0, len(forks_url)):
        contributors_repo = contributors_repo + get_data(url=forks_url[ix], params=params)
    contributors_repo = [x['login'] for x in contributors_repo]
    contributors_repo = list(set(contributors_repo))
    print(f"Total developers: {len(contributors_repo)}")

    url = 'https://api.github.com/repos/{}/subscribers'.format(repo)
    params = {'per_page': per_page, 'page': page}
    subscribers = get_data(url=url, params=params)
    subscribers = [x['login'] for x in subscribers]
    print(f"Total watchers: {len(subscribers)}")

    url = 'https://api.github.com/repos/{}/issues'.format(repo)
    params = {'per_page': per_page, 'page': page, 'state': 'all'}
    issuers = get_data(url=url, params=params)
    issuers = [x['user']['login'] for x in issuers]
    issuers = list(set(issuers))
    print(f"Total issuers: {len(issuers)}")

    total = stargazers + contributors + subscribers + issuers
    total = list(set(total))
    print(f"Total FIWARE users for the repo {repo}: {len(total)}")

    print('\n')

    return total, contributors_repo


def get_data(url, params):
    stargazers = list()
    headers = {
        'Accept': 'application/vnd.github.v3+json',
        'Authorization': 'bearer <your-github-token>'
    }

    while True:
        response = requests.get(url, headers=headers, params=params)

        if response.status_code == 403:
            print('   Rate limit errors, waiting for 1 hour...')
            time.sleep(3610)  # Sleep for 3610 seconds = 1h 10s
            print("   Awakening")

        if response.status_code != 200:
            print(f"Error: {response.status_code} - {response.json()}")
            break
        
        data = response.json()
        if not data:
            break  # No more stargazers to fetch
        
        stargazers.extend(data)
        
        # Check the Link header for pagination
        if 'Link' in response.headers:
            links = response.headers['Link']
            if 'rel="next"' not in links:
                break  # No more pages to fetch
        else:
            break  # No pagination information
        
        params['page'] = params['page'] + 1  # Move to the next page

    return stargazers


if __name__ == "__main__":
    users = list()
    contributors = list()

    with open('repos.json') as f:
        repos = json.load(f)
        print(repos)

    for x in repos:
        aux1, aux2 = get_statistics(x)
        users = users + aux1
        contributors = contributors + aux2

    users = list(set(users))
    contributors = list(set(contributors))

    print(f"Total FIWARE users: {len(users)}")
    print(f"Total FIWARE developers: {len(contributors)}")

    print('\n')
