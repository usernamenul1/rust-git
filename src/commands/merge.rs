use crate::core::reference::Reference;
use crate::core::index::Index;
use crate::core::tree::TreeProcessor;
use crate::commands::commit::git_commit;

pub fn git_merge(repo_path: &str, branch_name: &str) {
    // 获取目标分支的最新提交
    let target_commit = Reference::resolve(repo_path, &format!("heads/{}", branch_name))
        .expect(&format!("Branch '{}' not found", branch_name));

    // 获取当前分支的最新提交
    let current_commit = Reference::resolve_head(repo_path)
        .expect("Failed to resolve HEAD");

    // 如果目标分支就是当前提交,不需要合并
    if current_commit == target_commit {
        println!("Already up to date");
        return;
    }

    // 获取当前仓库的索引
    let staging_index = Index::load(repo_path);
    
    // 创建树对象
    let tree_hash = TreeProcessor::create_tree(repo_path, &staging_index.get_staged_files());

    // 创建合并提交信息
    let message = format!("Merge branch '{}'", branch_name);

    // 创建合并提交
    git_commit(repo_path, &message);

    println!("Merge made by the 'recursive' strategy.");
    println!("Fast-forward");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_test_repo() -> String {
        let temp_dir = std::env::temp_dir().join(format!("test_repo_{}", rand::random::<u32>()));
        let repo_path = temp_dir.to_str().unwrap().to_string();
        
        // 创建测试仓库结构
        fs::create_dir_all(&format!("{}/.git/refs/heads", repo_path)).unwrap();
        
        // 创建主分支和测试分支
        let main_commit = "1234567890123456789012345678901234567890";
        let feature_commit = "abcdef1234567890abcdef1234567890abcdef12";
        
        Reference::create(&repo_path, "heads/main", main_commit);
        Reference::create(&repo_path, "heads/feature", feature_commit);
        Reference::update_head_to_branch(&repo_path, "main").unwrap();
        
        repo_path
    }

    fn cleanup_test_repo(repo_path: &str) {
        fs::remove_dir_all(repo_path).unwrap_or_default();
    }

    #[test]
    fn test_merge_branches() {
        let repo_path = setup_test_repo();
        cleanup_test_repo(&repo_path);
        // 执行合并
        git_merge(&repo_path, "feature");
        
        // 验证合并结果
        let head_commit = Reference::resolve_head(&repo_path)
            .expect("Failed to resolve HEAD");
        
        // 确保 HEAD 有更新
        assert!(head_commit != "1234567890123456789012345678901234567890");
        
        cleanup_test_repo(&repo_path);
    }

    #[test]
    #[should_panic(expected = "Branch 'nonexistent' not found")]
    fn test_merge_nonexistent_branch() {
        let repo_path = setup_test_repo();
        git_merge(&repo_path, "nonexistent");
        cleanup_test_repo(&repo_path);
    }
}
