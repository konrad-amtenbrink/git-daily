mod git;
mod utils;

use clap::Parser;

#[derive(Parser)]
#[clap(
    author = "Konrad Amtenbrink",
    version,
    about = "A simple tool to get the latest git history"
)]
struct Args {
    #[clap(long, short, action)]
    user: Option<String>,
}

fn main() {
    let args = Args::parse();

    let repo = git::get_repo();

    git::fetch(&repo).expect("could not fetch repository");

    let mut revwalk = git::get_revwalk(&repo).expect("could not get revwalk");

    let commits = git::commit::get_commits(&mut revwalk, &repo, &args.user);

    utils::pretty_print_commits(commits);
}
