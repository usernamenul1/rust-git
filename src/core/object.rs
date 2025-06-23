use crate::utils::fs::{create_dir, write_file};
use crate::utils::hash::sha1;

/// Git 对象枚举
///
/// 表示 Git 中的三种主要对象类型
pub enum Object {
    /// 表示文件内容的 Blob 对象
    Blob(String),
    /// 表示目录结构的 Tree 对象
    Tree(String),
    /// 表示版本提交的 Commit 对象
    Commit(String),
}

impl Object {
    /// 保存 Git 对象到仓库
    ///
    /// 根据对象类型生成适当的对象内容，计算哈希值，并将对象写入仓库
    ///
    /// # 参数
    ///
    /// * `repo_path` - 仓库路径
    ///
    /// # 返回值
    ///
    /// 返回对象的 SHA-1 哈希值
    pub fn save(&self, repo_path: &str) -> String {
        let raw_data = match self {
            Object::Blob(data) => format!("blob {}\0{}", data.len(), data),
            Object::Tree(data) => format!("tree {}\0{}", data.len(), data),
            Object::Commit(data) => format!("commit {}\0{}", data.len(), data),
        };

        let hash = sha1(&raw_data);

        let dir = format!("{}/.git/objects/{}", repo_path, &hash[0..2]);
        create_dir(&dir);
        write_file(&format!("{}/{}", dir, &hash[2..]), raw_data);
        hash
    }
}
