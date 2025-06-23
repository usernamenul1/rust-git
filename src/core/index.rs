use std::collections::HashSet;
use std::io::Write;

use crate::utils::fs::read_file;

/// Git 索引（暂存区）结构体
///
/// 管理 Git 暂存区中的文件状态
pub struct Index {
    /// 仓库路径
    repo_path: String,
    /// 已暂存文件的集合
    staged_files: HashSet<String>,
}

impl Index {
    /// 加载仓库的索引
    ///
    /// 如果索引文件存在，则加载其内容；否则创建一个新的空索引
    ///
    /// # 参数
    ///
    /// * `repo_path` - 仓库路径
    ///
    /// # 返回值
    ///
    /// 返回加载或新建的 Index 实例
    pub fn load(repo_path: &str) -> Self {
        let index_file = format!("{}/.git/index", repo_path);

        // Check if the index file exists
        if !std::path::Path::new(&index_file).exists() {
            // If it doesn't exist, create a new index
            std::fs::File::create(&index_file).expect("Failed to create index file");
        } else {
            // If it exists, load the index from the file
            let contents = read_file(&index_file).expect("Failed to read index file");
            let staged_files: HashSet<String> = contents.lines().map(|s| s.to_string()).collect();
            return Index {
                repo_path: repo_path.to_string(),
                staged_files,
            };
        }
        Index {
            repo_path: repo_path.to_string(),
            staged_files: HashSet::new(),
        }
    }

    /// 暂存文件
    ///
    /// 将指定文件添加到暂存区
    ///
    /// # 参数
    ///
    /// * `file_path` - 要暂存的文件路径
    pub fn stage_file(&mut self, file_path: &str) {
        // Add the file to the staged files set
        self.staged_files.insert(file_path.to_string());
        // Write the staged files to the index file
        self.persist();
    }

    /// 取消暂存文件
    ///
    /// 从暂存区移除指定文件
    ///
    /// # 参数
    ///
    /// * `file_path` - 要取消暂存的文件路径
    pub fn unstage_file(&mut self, file_path: &str) {
        // Remove the file from the staged files set
        self.staged_files.remove(file_path);
        // Write the staged files to the index file
        self.persist();
    }

    /// 将暂存区状态持久化到索引文件
    ///
    /// # Panics
    ///
    /// 当创建索引文件或写入内容失败时会触发 panic
    fn persist(&self) {
        // Write the staged files to the index file
        let index_file = format!("{}/.git/index", self.repo_path);
        let mut file = std::fs::File::create(&index_file).expect("Failed to create index file");
        for file_path in &self.staged_files {
            writeln!(file, "{}", file_path).expect("Failed to write to index file");
        }
    }

    /// 获取所有暂存文件的列表
    ///
    /// # 返回值
    ///
    /// 返回以逗号分隔的暂存文件列表字符串
    pub fn get_staged_files(&self) -> String {
        // Return the staged files as a comma-separated string
        self.staged_files
            .iter()
            .cloned()
            .collect::<Vec<String>>()
            .join(", ")
    }
}
