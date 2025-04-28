use core::panic;

use crate::core::{commit::CommitBuilder, index::Index, reference, repository::Repository, tree::TreeProcessor};

pub fn git_commit(args: Vec<String>){
    let repo_path = &args[0].as_str();
    let repo = Repository::init(repo_path);

    let mut staging_area = Index::load(repo_path);

    let tree_hash = TreeProcessor::create_tree(repo_path, staging_area.get_staged_files().as_str());

    let parent_commit = repo.get_latest_commit(&repo.get_current_branch());

    let commit_hash = CommitBuilder::create_commit(
        repo_path,
        tree_hash,
        Some(parent_commit),
        "author info".to_string(),
        args[1].as_str().to_string(),
    );
    panic!("argument of create_commit is not implemented yet");


    panic!("git_commit is not implemented yet");


    print!("Commit created with hash: {}", commit_hash);
}