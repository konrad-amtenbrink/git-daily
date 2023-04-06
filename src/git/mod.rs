use std::env::current_dir;

use anyhow::Result;
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};

pub mod commit;

pub(crate) fn fetch(repo: &Repository) -> Result<(), git2::Error> {
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

pub(crate) fn get_repo() -> Repository {
    let path = current_dir().expect("could not find current directory");
    let repo = Repository::open(path).expect("could not open current repository");
    return repo;
}

pub(crate) fn get_revwalk(repo: &Repository) -> Result<git2::Revwalk, git2::Error> {
    let main_branch = repo.find_branch("main", git2::BranchType::Local)?;
    let main_branch_oid = main_branch.get().target().unwrap();

    let mut revwalk = repo.revwalk()?;
    revwalk.push(main_branch_oid)?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    return Ok(revwalk);
}
