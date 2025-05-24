pub struct Reference;

impl Reference {
    pub fn create(repo_path: &str, ref_name: &str, target_hash: &str) {
        // println!("ref_name : {}", ref_name);
        let ref_path = format!("{}/.git/refs/{}", repo_path, ref_name);
        // println!("Creating reference at: {}", ref_path);
        // 正确的引用文件内容只包含哈希值，不带 "ref:" 前缀
        let ref_content = format!("{}\n", target_hash);
        std::fs::write(ref_path, ref_content).expect("Failed to write reference");
    }

    pub fn delete(repo_path: &str, ref_name: &str) {
        let ref_path = format!("{}/.git/refs/{}", repo_path, ref_name);
        std::fs::remove_file(ref_path).expect("Failed to delete reference");
    }

    pub fn resolve(repo_path: &str, ref_name: &str) -> Option<String> {
        if std::path::Path::new(&format!("{}/.git/refs/{}", repo_path, ref_name)).exists() {
            let ref_content =
                std::fs::read_to_string(format!("{}/.git/refs/{}", repo_path, ref_name))
                    .expect("Failed to read reference");
            return Some(ref_content.trim().to_string());
        }
        None
    }

    // 将 HEAD 设置为指向分支
    pub fn update_head_to_branch(repo_path: &str, branch_name: &str) -> std::io::Result<()> {
        let head_path = format!("{}/.git/HEAD", repo_path);
        let content = format!("ref: refs/heads/{}\n", branch_name);
        std::fs::write(head_path, content)
    }

    // 将 HEAD 设置为直接指向提交（分离状态）
    pub fn update_head_to_commit(repo_path: &str, commit_hash: &str) -> std::io::Result<()> {
        let head_path = format!("{}/.git/HEAD", repo_path);
        std::fs::write(head_path, format!("{}\n", commit_hash))
    }

    // 获取 HEAD 指向的提交哈希，无论何种状态
    pub fn resolve_head(repo_path: &str) -> Option<String> {
        let head_path = format!("{}/.git/HEAD", repo_path);
        if let Ok(content) = std::fs::read_to_string(head_path) {
            let content = content.trim();
            if content.starts_with("ref: refs/heads/") {
                // HEAD 指向分支，需要解析分支引用
                let branch = content.trim_start_matches("ref: refs/heads/");
                return Self::resolve(repo_path, &format!("heads/{}", branch));
            } else {
                // 分离 HEAD 状态，直接返回提交哈希
                return Some(content.to_string());
            }
        }
        None
    }

    pub fn is_head_detached(repo_path: &str) -> bool {
        let head_path = format!("{}/.git/HEAD", repo_path);
        if let Ok(content) = std::fs::read_to_string(head_path) {
            !content.starts_with("ref: ")
        } else {
            false // 文件不存在或无法读取，默认为非分离状态
        }
    }
}
