use std::env::current_dir;

use anyhow::Result;
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};

fn fetch(repo: &Repository) -> Result<(), git2::Error> {
    let mut remote = repo.find_remote("origin")?;
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key_from_agent(username_from_url.unwrap())
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    remote.fetch(&["main"], Some(&mut fetch_options), None)?;

    Ok(())
}

fn get_repo() -> Repository {
    let path = current_dir().expect("could not find current directory");
    let repo = Repository::open(path).expect("could not open current repository");
    return repo;
}

fn main() {
    let repo = get_repo();

    fetch(&repo).expect("could not fetch repository");

    let head = repo.head().expect("could not find head");

    let reflog = repo
        .reflog(&head.name().expect("could not find head"))
        .unwrap();

    let commit = reflog.get(0).expect("could not get repository history");
    let raw_commit_msg = commit
        .message()
        .expect("could not retrieve commit messsage");

    let split_index = raw_commit_msg
        .find(":")
        .expect("could not retrieve commit messsage")
        + 2;

    let (_, commit_msg) = &raw_commit_msg.split_at(split_index);
    println!("{}", commit_msg)
}
