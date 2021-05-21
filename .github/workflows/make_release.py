import os
import sys
import git
import termcolor
import requests
import json
import pathlib


api_url = 'https://api.github.com'


def create_release(repo_path, tag):
    url = f'{api_url}/repos/{repo_path}/releases'
    data = {'tag_name': tag, 'draft': True}
    headers = {
        'Accept': 'application/vnd.github.v3+json',
        'Authorization': f'Bearer {bearer}'
    }
    resp = requests.post(url, data=json.dumps(data), headers=headers)
    if resp.status_code != 201:
        print(termcolor.colored(
            f'Failed to create draft release for {tag}', 'red'))
        sys.exit(1)
    return json.loads(resp.content)


def get_realease_for_tag(releases, tag):
    for release in releases:
        if release['tag_name'] == tag:
            return release
    return None


if __name__ == "__main__":
    # Args check
    if len(sys.argv) != 3:
        print(f'Usage: {sys.argv[0]} <file_to_upload> <bearer>')
        sys.exit(1)

    repo_path = 'RedDocMD/cork'
    file_path = pathlib.PurePath(sys.argv[1])
    bearer = sys.argv[2]

    print(f'Starting asset upload from {file_path} to {repo_path} ...')

    # Find out the last tag
    repo = git.Repo('.')
    tags = repo.tags
    if len(tags) == 0:
        print(termcolor.colored('Failed to find any tags', 'red'))
        sys.exit(1)
    tag = str(tags[0])

    print(f'Git tag: {tag}')

    # Check to the Github API to find if the release has been created
    print(f'Checking for existing release ... ')
    url = f'{api_url}/repos/{repo_path}/releases'
    headers = {
        'Accept': 'application/vnd.github.v3+json',
        'Authorization': f'Bearer {bearer}'
    }
    resp = requests.get(url, headers=headers)
    if resp.status_code != 200:
        print(termcolor.colored('Failed to retrieve releases list!', 'red'))
        sys.exit(1)
    cont = json.loads(resp.content)
    release = get_realease_for_tag(cont, tag)

    # If it has not been created, create it
    if release is None:
        print('No release found.')
        print('Creating new release ...')
        release = create_release(repo_path, tag)

    upload_url = release['upload_url']
    cut_idx = upload_url.index('{')
    upload_url = upload_url[0:cut_idx]

    # Now upload the file
    try:
        with open(file_path, 'rb') as f:
            print(f'Uploading {file_path} to {upload_url} ...')
            file_name = file_path.name
            payload = {'name': file_name}
            headers = {
                'Accept': 'application/vnd.github.v3+json',
                'Authorization': f'Bearer {bearer}',
                'Content-Type': 'application/octet-stream'
            }
            data = f.read()
            url = f'{upload_url}?name={file_name}'
            resp = requests.post(url, data=data, headers=headers)
            if resp.status_code != 201:
                print(termcolor.colored(f"Failed to upload {file_path}", 'red'))
                sys.exit(1)
    except FileNotFoundError:
        print(termcolor.colored(f'Failed to open {file_path}', 'red'))
        sys.exit(1)

    print(termcolor.colored(f'Successfully uploaded {file_path}', 'green'))
