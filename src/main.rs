mod git;

fn main() {
    let repo = git::get_repo();

    git::fetch(&repo).expect("could not fetch repository");

    let head = repo.head().expect("could not find head");

    let reflog = repo
        .reflog(&head.name().expect("could not find head"))
        .unwrap();

    git::get_commit_messages(&reflog);
}
