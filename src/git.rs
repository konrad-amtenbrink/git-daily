use crate::utils::convert_time;
use std::env::current_dir;

use anyhow::Result;
use git2::{Commit, Cred, FetchOptions, RemoteCallbacks, Repository};

pub struct CommitData {
    pub commit_message: String,
    pub commit_date: String,
    pub commit_author: String,
}

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

pub(crate) fn get_commits(revwalk: &mut git2::Revwalk, repo: &Repository) -> Vec<CommitData> {
    let commits = revwalk
        .take(10)
        .filter_map(|oid| Some(repo.find_commit(oid.expect("could not find commit"))));
    let mut commit_data_vec = Vec::new();
    commits.for_each(|commit| {
        let commit_data = get_commit_data(&commit.expect("could not find commit"));
        commit_data_vec.push(commit_data);
    });
    return commit_data_vec;
}

fn get_commit_data(commit: &Commit) -> CommitData {
    let mut commit_message = commit
        .message()
        .expect("could not retrieve commit messsage")
        .to_string();

    commit_message.truncate(commit_message.len() - 1);

    let committer = commit.committer();

    let commit_author = committer.name().expect("could not retrieve commit author");

    let commit_date: String = convert_time(commit.time().seconds());

    let commit_data = CommitData {
        commit_message: commit_message,
        commit_date: commit_date.to_string(),
        commit_author: commit_author.to_string(),
    };
    return commit_data;
}
