use crate::git::commit::CommitData;
use colored::Colorize;

use chrono::{DateTime, Duration, NaiveDateTime, Utc};

pub(crate) fn pretty_print_commits(commits: Vec<CommitData>) {
    for commit in commits {
        println!(
            "{0: <20} {1: <50} {2: <20}",
            commit.commit_author.bright_blue(),
            commit.commit_message.yellow(),
            commit.commit_date.green(),
        );
    }
}

pub(crate) fn convert_time(timestamp: i64) -> String {
    // FIXME remove deprecated function
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    let now: DateTime<Utc> = Utc::now();
    let diff = now - datetime;

    return get_formatted_duration(diff);
}

fn get_formatted_duration(diff: Duration) -> String {
    let mut duration: i64 = diff.num_seconds();
    let mut unit = "seconds";

    if duration > 60 {
        duration = diff.num_minutes();
        unit = "minutes";
    }

    if duration > 60 {
        duration = diff.num_hours();
        unit = "hours";
    }

    if duration > 24 && unit == "hours" {
        duration = diff.num_days();
        unit = "days";
    }

    return format!("{} {} ago", duration, unit);
}
