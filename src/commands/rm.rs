use crate::core::index::Index;

/// 执行 git rm 命令
///
/// 从工作目录和索引中移除指定文件
///
/// # 参数
///
/// * `repo_path` - 仓库路径
/// * `file_path` - 要移除的文件路径
/// * `force_mode` - 是否强制移除（即使有未提交的修改）
///
/// # Panics
///
/// 当文件删除失败时会触发 panic
pub fn git_rm(repo_path: &str, file_path: &str, force_mode: bool) {
    let mut staging_area = Index::load(repo_path);

    if force_mode {
        // remove the file from the working directory
        std::fs::remove_file(file_path).expect("Failed to remove file");
    } else {
        // remove the file from the working directory
        std::fs::remove_file(file_path).expect("Failed to remove file");
    }

    // save the index
    staging_area.unstage_file(file_path);

    // println!("Removed file from staging area: {}", file_path);
}
