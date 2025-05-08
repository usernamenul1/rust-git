use crate::commands::merge;
use crate::core::commit::CommitBuilder;
use crate::core::reference::Reference;
use crate::core::tree::{self, TreeProcessor};

pub fn git_merge(repo_path: &str, branch_name: &str) {
    //     // 1. 获取当前分支的最新提交哈希
    //  let current_commit_hash = 获取当前分支提交(repo_path, current_branch);
    //  // 2. 获取目标分支的最新提交哈希
    // let target_commit_hash = 获取目标分支提交(repo_path, branch_name);
    //  // 3. 创建一个新的合并提交
    // let merge_commit_hash = 创建合并提交(
    //  repo_path,
    // "tree_hash_placeholder",
    // Some(current_commit_hash),
    // "Author Name".to_string(),
    // // 仓库路径
    // // 提交树哈希
    // // 当前分支提交哈希
    // // 作者信息
    // 格式化"Merge branch [branch_name]" // 合并信息
    // );
    //  // 4. 更新当前分支的引用，指向新的合并提交
    // 更新当前分支引用(repo_path, current_branch, &merge_commit_hash);
    //  // 5. 用户反馈
    // 打印"Merge branch [branch_name] into [current_branch]";
    let current_branch = Reference::resolve(repo_path, "HEAD").unwrap_or_else(|| {
        eprintln!("Error: Unable to resolve current branch");
        std::process::exit(1);
    });
    let target_branch = Reference::resolve(repo_path, branch_name).unwrap_or_else(|| {
        eprintln!("Error: Unable to resolve target branch");
        std::process::exit(1);
    });
    let current_commit_hash = Reference::resolve(repo_path, &current_branch).unwrap_or_else(|| {
        eprintln!("Error: Unable to resolve current commit");
        std::process::exit(1);
    });
    let target_commit_hash = Reference::resolve(repo_path, &target_branch).unwrap_or_else(|| {
        eprintln!("Error: Unable to resolve target commit");
        std::process::exit(1);
    });
    panic!("not implemented yet");
    // let tree_hash = TreeProcessor::create_tree(repo_path, merged_files);
    // let merge_commit_hash = CommitBuilder::create_commit(
    //     repo_path,
    //     tree_hash,
    //     Some(current_commit_hash),
    //     "Author Name".to_string(),
    //     format!("Merge branch {}", branch_name),
    // );

    // Reference::create(repo_path, &current_branch, &merge_commit_hash);

    // println!("Merge branch {} into {}", branch_name, current_branch);
}
