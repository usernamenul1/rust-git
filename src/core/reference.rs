/// Git 引用处理结构体
///
/// 提供处理 Git 引用（如分支、标签）的功能
pub struct Reference;

impl Reference {
    /// 创建一个新的引用
    ///
    /// # 参数
    ///
    /// * `repo_path` - 仓库路径
    /// * `ref_name` - 引用名称，相对于 refs/ 目录
    /// * `target_hash` - 引用指向的目标哈希值
    ///
    /// # Panics
    ///
    /// 当写入引用文件失败时会触发 panic
    pub fn create(repo_path: &str, ref_name: &str, target_hash: &str) {
        // println!("ref_name : {}", ref_name);
        let ref_path = format!("{}/.git/refs/{}", repo_path, ref_name);
        // println!("Creating reference at: {}", ref_path);
        // 正确的引用文件内容只包含哈希值，不带 "ref:" 前缀
        let ref_content = format!("{}\n", target_hash);
        std::fs::write(ref_path, ref_content).expect("Failed to write reference");
    }

    /// 删除指定的引用
    ///
    /// # 参数
    ///
    /// * `repo_path` - 仓库路径
    /// * `ref_name` - 要删除的引用名称，相对于 refs/ 目录
    ///
    /// # Panics
    ///
    /// 当删除引用文件失败时会触发 panic
    pub fn delete(repo_path: &str, ref_name: &str) {
        let ref_path = format!("{}/.git/refs/{}", repo_path, ref_name);
        std::fs::remove_file(ref_path).expect("Failed to delete reference");
    }

    /// 解析引用，获取其指向的提交哈希值
    ///
    /// # 参数
    ///
    /// * `repo_path` - 仓库路径
    /// * `ref_name` - 引用名称，相对于 refs/ 目录
    ///
    /// # 返回值
    ///
    /// 如果引用存在，返回 `Some(hash)`，否则返回 `None`
    pub fn resolve(repo_path: &str, ref_name: &str) -> Option<String> {
        if std::path::Path::new(&format!("{}/.git/refs/{}", repo_path, ref_name)).exists() {
            let ref_content =
                std::fs::read_to_string(format!("{}/.git/refs/{}", repo_path, ref_name))
                    .expect("Failed to read reference");
            return Some(ref_content.trim().to_string());
        }
        None
    }

    /// 将 HEAD 设置为指向指定分支
    ///
    /// # 参数
    ///
    /// * `repo_path` - 仓库路径
    /// * `branch_name` - 分支名称
    ///
    /// # 返回值
    ///
    /// 成功返回 `Ok(())`，失败返回 IO 错误
    pub fn update_head_to_branch(repo_path: &str, branch_name: &str) -> std::io::Result<()> {
        let head_path = format!("{}/.git/HEAD", repo_path);
        let content = format!("ref: refs/heads/{}\n", branch_name);
        std::fs::write(head_path, content)
    }

    /// 将 HEAD 设置为直接指向指定提交（分离状态）
    ///
    /// # 参数
    ///
    /// * `repo_path` - 仓库路径
    /// * `commit_hash` - 提交哈希值
    ///
    /// # 返回值
    ///
    /// 成功返回 `Ok(())`，失败返回 IO 错误
    pub fn update_head_to_commit(repo_path: &str, commit_hash: &str) -> std::io::Result<()> {
        let head_path = format!("{}/.git/HEAD", repo_path);
        std::fs::write(head_path, format!("{}\n", commit_hash))
    }

    /// 获取 HEAD 指向的提交哈希值
    ///
    /// 无论 HEAD 是指向分支还是直接指向提交，都能正确解析
    ///
    /// # 参数
    ///
    /// * `repo_path` - 仓库路径
    ///
    /// # 返回值
    ///
    /// 如果成功解析，返回 `Some(hash)`，否则返回 `None`
    pub fn resolve_head(repo_path: &str) -> Option<String> {
        let head_path = format!("{}/.git/HEAD", repo_path);
        if let Ok(content) = std::fs::read_to_string(head_path) {
            let content = content.trim();
            if content.starts_with("ref: refs/heads/") {
                // HEAD 指向分支，需要解析分支引用
                let branch = content.trim_start_matches("ref: refs/heads/");
                // println!("branch : {}", branch);
                return Self::resolve(repo_path, &format!("heads/{}", branch));
            } else {
                // 分离 HEAD 状态，直接返回提交哈希
                println!("detached head : {}", content);
                return Some(content.to_string());
            }
        }
        None
    }

    /// 检查 HEAD 是否处于分离状态
    ///
    /// HEAD 分离状态指 HEAD 直接指向提交，而不是通过分支间接指向
    ///
    /// # 参数
    ///
    /// * `repo_path` - 仓库路径
    ///
    /// # 返回值
    ///
    /// 如果 HEAD 处于分离状态则返回 `true`，否则返回 `false`
    pub fn is_head_detached(repo_path: &str) -> bool {
        let head_path = format!("{}/.git/HEAD", repo_path);
        if let Ok(content) = std::fs::read_to_string(head_path) {
            !content.starts_with("ref: ")
        } else {
            false // 文件不存在或无法读取，默认为非分离状态
        }
    }
}
