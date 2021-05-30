#![allow(dead_code)]

use crate::{
    error::Result,
    sync::{logwalker::Mode, utils, CommitId, LogWalker},
};
use scopetime::scope_time;

pub fn print_tree(repo_path: &str) -> Result<()> {
    scope_time!("print_tree");

    let repo = utils::repo(repo_path)?;

    let mut commits = Vec::new();
    {
        let mut walker = LogWalker::new(&repo).mode(Mode::AllRefs);
        walker.read(&mut commits, 1000)?;
    }

    println!("commits: {}", commits.len());
    for c in &commits {
        let commit = repo.find_commit((*c).into())?;
        let parents: Vec<String> = commit
            .parents()
            .into_iter()
            .map(|p| CommitId::new(p.id()).get_short_string())
            .collect();
        println!("{} ({:?})", c.get_short_string(), parents);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::{
        checkout_branch, create_branch,
        tests::{repo_init_empty, write_commit_file_at},
    };
    use git2::Time;
    // use pretty_assertions::assert_eq;

    #[test]
    fn test_smoke() {
        let (td, repo) = repo_init_empty().unwrap();
        let repo_path = td.path().to_string_lossy();

        let _c1 = write_commit_file_at(
            &repo,
            "test.txt",
            "",
            "c1",
            Time::new(1, 0),
        );

        let b1 = create_branch(&repo_path, "b1").unwrap();

        let _c2 = write_commit_file_at(
            &repo,
            "test2.txt",
            "",
            "c2",
            Time::new(2, 0),
        );

        let _b2 = create_branch(&repo_path, "b2").unwrap();

        let _c3 = write_commit_file_at(
            &repo,
            "test3.txt",
            "",
            "c3",
            Time::new(3, 0),
        );

        checkout_branch(&repo_path, &b1).unwrap();

        print_tree(&repo_path).unwrap();
    }
}
