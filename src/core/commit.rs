use crate::core::object::Object;

/// 提交创建器结构体
pub struct CommitBuilder;

impl CommitBuilder {
    /// 创建一个新的提交对象
    ///
    /// # 参数
    ///
    /// * `repo_path` - 仓库路径
    /// * `tree_hash` - 树对象的哈希值，表示提交的文件状态
    /// * `parent_hash` - 可选的父提交哈希值
    /// * `author_info` - 作者信息
    /// * `commit_messsage` - 提交消息
    ///
    /// # 返回值
    ///
    /// 返回新创建的提交对象的 SHA-1 哈希值
    pub fn create_commit(
        repo_path: &str,
        tree_hash: String,
        parent_hash: Option<String>,
        author_info: String,
        commit_messsage: String,
    ) -> String {
        //get current timestamp RFC 2822
        let timestamp = chrono::Utc::now().to_rfc2822();

        // panic!("CommitBuilder::create_commit is not implemented yet");
        let commit_content = format!(
            "tree {}\nparent {}\nauthor {}{}\n\n{}",
            tree_hash,
            match parent_hash {
                Some(hash) => format!("parent {}\n", hash),
                None => String::new(),
            },
            author_info,
            timestamp,
            commit_messsage
        );

        let commit_obj = Object::Commit(commit_content);
        commit_obj.save(repo_path)
    }
}
