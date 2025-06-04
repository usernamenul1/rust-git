use crate::core::index::Index;

/// 执行 git add 命令
///
/// 将指定文件添加到暂存区
///
/// # 参数
///
/// * `repo_path` - 仓库路径
/// * `file_path` - 要添加到暂存区的文件路径
pub fn git_add(repo_path: &str, file_path: &str) {
    // check if the file exists
    let mut staging_area = Index::load(repo_path);

    // save the index
    staging_area.stage_file(file_path);

    // println!("Added {} to staging area", file_path);
}
