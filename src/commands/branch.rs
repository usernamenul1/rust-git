use crate::core::reference::Reference;
pub fn git_branch(
    repo_path: &str,
    branch_name: &str, 
    delete: bool
) {
    if delete {
        // 检查是否试图删除当前分支
        if !Reference::is_head_detached(repo_path) {
            if let Ok(content) = std::fs::read_to_string(format!("{}/.git/HEAD", repo_path)) {
                let current_branch = content.trim()
                    .trim_start_matches("ref: refs/heads/")
                    .trim_end();
                if current_branch == branch_name {
                    panic!("Cannot delete the current branch");
                }
            }
        }
        
        // 删除分支引用
        Reference::delete(repo_path, &format!("heads/{}", branch_name));
        // println!("Deleted branch '{}'", branch_name);
    } else {
        // 获取当前 HEAD 指向的提交
        let head_commit = Reference::resolve_head(repo_path)
            .expect("Failed to resolve HEAD");
        
        // 检查分支是否已存在
        if Reference::resolve(repo_path, &format!("heads/{}", branch_name)).is_some() {
            panic!("Branch '{}' already exists", branch_name);
        }
        
        // 创建新分支
        Reference::create(repo_path, &format!("heads/{}", branch_name), &head_commit);
        // println!("Created branch '{}'", branch_name);
    }
}

// ...existing code...

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_test_repo() -> String {
        let temp_dir = std::env::temp_dir().join(format!("test_repo_{}", rand::random::<u32>()));
        let repo_path = temp_dir.to_str().unwrap().to_string();
        
        // 创建测试仓库结构
        fs::create_dir_all(&format!("{}/.git/refs/heads", repo_path)).unwrap();
        
        // 创建一个初始提交
        let initial_commit = "1234567890123456789012345678901234567890";
        Reference::create(&repo_path, "heads/main", initial_commit);
        Reference::update_head_to_branch(&repo_path, "main").unwrap();
        
        repo_path
    }

    fn cleanup_test_repo(repo_path: &str) {
        fs::remove_dir_all(repo_path).unwrap_or_default();
    }

    #[test]
    fn test_create_branch() {
        let repo_path = setup_test_repo();
        
        // 测试创建分支
        git_branch(&repo_path, "test-branch", false);
        
        // 验证分支是否创建成功
        assert!(Reference::resolve(&repo_path, "heads/test-branch").is_some());
        
        cleanup_test_repo(&repo_path);
    }

    #[test]
    fn test_delete_branch() {
        let repo_path = setup_test_repo();
        
        // 先创建一个分支
        git_branch(&repo_path, "test-branch", false);
        
        // 测试删除分支
        git_branch(&repo_path, "test-branch", true);
        
        // 验证分支是否已删除
        assert!(Reference::resolve(&repo_path, "heads/test-branch").is_none());
        
        cleanup_test_repo(&repo_path);
    }

    #[test]
    #[should_panic(expected = "Branch 'test-branch' already exists")]
    fn test_create_existing_branch() {
        let repo_path = setup_test_repo();
        
        // 创建分支
        git_branch(&repo_path, "test-branch", false);
        
        // 尝试创建同名分支，应该panic
        git_branch(&repo_path, "test-branch", false);
        
        cleanup_test_repo(&repo_path);
    }

    #[test]
    #[should_panic(expected = "Cannot delete the current branch")]
    fn test_delete_current_branch() {
        let repo_path = setup_test_repo();
        
        // 尝试删除当前分支（main），应该panic
        git_branch(&repo_path, "main", true);
        
        cleanup_test_repo(&repo_path);
    }
}