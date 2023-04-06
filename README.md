<!-- markdownlint-configure-file {
  "MD013": {
    "code_blocks": false,
    "tables": false
  },
  "MD033": false,
  "MD041": false
} -->

<div align="center">

# git-daily

git-daily is a slimmed down **git history command** written in Rust, inspired by [git-standup](https://github.com/kamranahmedse/git-standup).

[Usage](#usage) •
[Installation](#installation) •
[Options](#options)
</div>

## Usage

```sh
git-daily                   # show last commits in a clean structured way
```

You can also use `git daily` if you followed step 2 of the [installation instructions](#installation).

## Installation

### *Step 1: Install git-daily*
Right now, git-daily is only installable via homebrew

To install git-daily, run these commands in your terminal:

```sh
brew tap konrad-amtenbrink/tap
brew update && brew install git-daily
```


### *Step 2 (Optional): Add git-daily as git command*
To add `git-daily` as a git command, run this command in your terminal:

```sh
echo "export PATH=$PATH:/usr/local/Cellar/git-daily/0.1.0/bin/" >> ~/.zshrc
```

Be aware that this directly adds `git-daily` to your path so that git recognizes it.

## Options
You can pass options to filter the recent commit history.

```sh
git-daily [-u <username>] # matches all commits that contain the username as author
```
### Example
In order for this to work, the username does not have to be complete.

```sh
git daily -u konrad # also matches all commits authored by `Konrad Amtenbrink`
```
