use crate::utils::convert_time;

use git2::{Commit, Repository};

pub struct CommitData {
    pub commit_message: String,
    pub commit_date: String,
    pub commit_author: String,
}

pub(crate) fn get_commits(
    revwalk: &mut git2::Revwalk,
    repo: &Repository,
    user: &Option<String>,
) -> Vec<CommitData> {
    let user_name: String = user.as_ref().map_or("".to_string(), |s| s.to_string());

    let commits = revwalk
        .take(50)
        .filter_map(|oid| Some(repo.find_commit(oid.expect("could not find commit"))));

    let mut commit_data_vec = Vec::new();
    commits.for_each(|commit| {
        let commit_data = get_commit_data(&commit.expect("could not find commit"));

        if contains_user(&commit_data, &user_name) {
            commit_data_vec.push(commit_data);
        }
    });

    return commit_data_vec;
}

fn contains_user(commit_data: &CommitData, user_name: &String) -> bool {
    if user_name.is_empty() {
        return true;
    }

    return commit_data
        .commit_author
        .to_ascii_lowercase()
        .contains(&user_name.to_ascii_lowercase());
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
