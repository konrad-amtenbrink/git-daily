mod git;
mod utils;

fn main() {
    let repo = git::get_repo();

    git::fetch(&repo).expect("could not fetch repository");

    let mut revwalk = git::get_revwalk(&repo).expect("could not get revwalk");

    let commits = git::commit::get_commits(&mut revwalk, &repo);

    utils::pretty_print_commits(commits);
}
