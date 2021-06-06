#![allow(unused_imports, dead_code, clippy::unwrap_used)]

use crate::{
    error::Result,
    sync::{utils, CommitId, LogWalker},
};
use scopetime::scope_time;

struct State {
    // threads: Vec,
}

pub fn print_tree(repo_path: &str) -> Result<()> {
    scope_time!("print_tree");

    let repo = utils::repo(repo_path)?;

    let mut commits = Vec::new();
    {
        let mut walker = LogWalker::new(&repo, 1000)?;
        walker.read(&mut commits)?;
    }

    println!("commits: {}", commits.len());
    for c in &commits {
        let commit = repo.find_commit((*c).into())?;

        // let upwards

        let parents: Vec<String> = commit
            .parents()
            .into_iter()
            .map(|p| CommitId::new(p.id()).get_short_string())
            .collect();

        println!(
            "{} ({:?}) {}",
            c.get_short_string(),
            parents,
            commit.message().unwrap()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use git2::Time;

    use super::*;
    use crate::sync::{
        checkout_branch, commit, create_branch,
        tests::{commit_at, repo_init_empty, write_commit_file},
    };
    // use pretty_assertions::assert_eq;

    fn gittime(s: i64) -> Time {
        Time::new(s, 0)
    }

    #[test]
    fn test_smoke() {
        // ●  [b1] c4
        // │ ●  [b2] c3
        // ●─╯  c2
        // ●  [master] c1

        let (td, _repo) = repo_init_empty().unwrap();
        let path = td.path().to_string_lossy();

        let _c1 = commit_at(&path, "c1", gittime(0));

        let b1 = create_branch(&path, "b1").unwrap();

        let _c2 = commit_at(&path, "c2", gittime(1));

        let _b2 = create_branch(&path, "b2").unwrap();

        let _c3 = commit_at(&path, "c3", gittime(2));

        checkout_branch(&path, &b1).unwrap();

        let _c4 = commit_at(&path, "c4", gittime(3));

        print_tree(&path).unwrap();
    }
}
