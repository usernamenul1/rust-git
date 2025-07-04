use crate::core::reference::Reference;

/// 执行 git checkout 命令
///
/// 切换到指定的分支或提交，或者创建新分支并切换到它
///
/// # 参数
///
/// * `repo_path` - 仓库路径
/// * `target` - 目标分支名或提交哈希值
/// * `new_branch` - 是否创建新分支（-b 选项）
///
/// # Panics
///
/// 当分支或提交不存在，或者分支已存在但使用了 new_branch 选项时会触发 panic
pub fn git_checkout(repo_path: &str, target: &str, new_branch: bool) {
    if new_branch {
        let branch_name = target;
        if Reference::resolve(repo_path, &format!("heads/{}", branch_name)).is_some() {
            panic!("Branch '{}' already exists", branch_name);
        }
        Reference::create(repo_path, &format!("heads/{}", branch_name), target);
        // Reference::update_head_to_branch(repo_path, &branch_name)
        //     .expect("Failed to update HEAD to new branch");
        // return;
    }
    if let Some(_) = Reference::resolve(repo_path, &format!("heads/{}", target)) {
        // 目标是分支，更新 HEAD 指向该分支
        Reference::update_head_to_branch(repo_path, target)
            .expect("Failed to update HEAD to branch");
    } else if target.len() == 40 && target.chars().all(|c| c.is_ascii_hexdigit()) {
        // 目标可能是提交哈希
        Reference::update_head_to_commit(repo_path, target)
            .expect("Failed to update HEAD to commit");
    } else {
        panic!("Branch or commit not found: {}", target);
    }

    /*
    println!("Switched to {}", if target.len() == 40 {
        format!("commit {}", &target[..8])
    } else {
        format!("branch '{}'", target)
    });
    */
}

//testcase

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_test_repo() -> String {
        let temp_dir = std::env::temp_dir().join(format!("test_repo_{}", rand::random::<u32>()));
        let repo_path = temp_dir.to_str().unwrap().to_string();

        // 创建测试仓库结构
        fs::create_dir_all(&format!("{}/.git/refs/heads", repo_path)).unwrap();

        // 创建一个初始提交和主分支
        let initial_commit = "1234567890123456789012345678901234567890";
        Reference::create(&repo_path, "heads/main", initial_commit);
        Reference::update_head_to_branch(&repo_path, "main").unwrap();

        // 创建一个测试分支
        let test_commit = "abcdef1234567890abcdef1234567890abcdef12";
        Reference::create(&repo_path, "heads/test-branch", test_commit);

        repo_path
    }

    fn cleanup_test_repo(repo_path: &str) {
        fs::remove_dir_all(repo_path).unwrap_or_default();
    }

    #[test]
    fn test_checkout_branch() {
        let repo_path = setup_test_repo();

        // 测试检出分支
        git_checkout(&repo_path, "test-branch",false);

        // 验证 HEAD 是否正确指向分支
        assert!(!Reference::is_head_detached(&repo_path));
        let head_content = fs::read_to_string(format!("{}/.git/HEAD", repo_path)).unwrap();
        assert!(head_content.contains("ref: refs/heads/test-branch"));

        cleanup_test_repo(&repo_path);
    }

    #[test]
    fn test_checkout_commit() {
        let repo_path = setup_test_repo();
        let commit_hash = "1234567890123456789012345678901234567890";

        // 测试检出特定提交
        git_checkout(&repo_path, commit_hash,false);

        // 验证是否处于分离 HEAD 状态
        assert!(Reference::is_head_detached(&repo_path));
        let head_content = fs::read_to_string(format!("{}/.git/HEAD", repo_path)).unwrap();
        assert_eq!(head_content.trim(), commit_hash);

        cleanup_test_repo(&repo_path);
    }

    #[test]
    #[should_panic(expected = "Branch or commit not found")]
    fn test_checkout_nonexistent_target() {
        let repo_path = setup_test_repo();

        // 测试检出不存在的目标
        git_checkout(&repo_path, "nonexistent-branch",false);

        cleanup_test_repo(&repo_path);
    }
}
