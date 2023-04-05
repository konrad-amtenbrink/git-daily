use crate::git::CommitData;

use chrono::{DateTime, NaiveDateTime, Utc};

pub(crate) fn pretty_print_commits(commits: Vec<CommitData>) {
    for commit in commits {
        print!(
            "({})   {} - {}\n",
            commit.commit_date, commit.commit_message, commit.commit_author,
        );
    }
}

pub(crate) fn convert_time(timestamp: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S");

    return newdate.to_string();
}
