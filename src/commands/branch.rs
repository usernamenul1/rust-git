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
        println!("Deleted branch '{}'", branch_name);
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
        println!("Created branch '{}'", branch_name);
    }
}
