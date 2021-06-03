#![allow(unused_imports, dead_code)]

use crate::{
    error::Result,
    sync::{logwalker::Mode, utils, CommitId, LogWalker},
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
        let mut walker = LogWalker::new(&repo);
        walker.read(&mut commits, 1000)?;
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
    use super::*;
    use crate::sync::{
        checkout_branch, create_branch,
        tests::{repo_init_empty, write_commit_file},
    };
    // use pretty_assertions::assert_eq;

    #[test]
    fn test_smoke() {
        // │ ●  [b1] c4
        // │ │ ●  [b2] c3
        // │ ●─╯  c2
        // ●─╯  [master] c1

        let (td, repo) = repo_init_empty().unwrap();
        let repo_path = td.path().to_string_lossy();

        let c1 = write_commit_file(&repo, "test.txt", "", "c1");
        dbg!(c1);

        let b1 = create_branch(&repo_path, "b1").unwrap();

        let c2 = write_commit_file(&repo, "test2.txt", "", "c2");
        dbg!(c2);

        let _b2 = create_branch(&repo_path, "b2").unwrap();

        let c3 = write_commit_file(&repo, "test3.txt", "", "c3");
        dbg!(c3);

        checkout_branch(&repo_path, &b1).unwrap();

        let c4 = write_commit_file(&repo, "test4.txt", "", "c4");
        dbg!(c4);

        print_tree(&repo_path).unwrap();
    }
}
