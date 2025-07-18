use crate::commands::commit::git_commit;
use crate::core::index::Index;
use crate::core::object::Object;
use crate::core::reference::Reference;
use crate::core::tree::TreeProcessor;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// 执行 git merge 命令
///
/// 合并指定分支到当前分支
///
/// # 参数
///
/// * `repo_path` - 仓库路径
/// * `branch_name` - 要合并的分支名称
///
/// # Panics
///
/// 当分支不存在或存在合并冲突时会触发 panic
pub fn git_merge(repo_path: &str, branch_name: &str) {
    // 获取目标分支的最新提交
    let target_commit = Reference::resolve(repo_path, &format!("heads/{}", branch_name))
        .expect(&format!("Branch '{}' not found", branch_name));

    // 获取当前分支的最新提交
    let current_commit = Reference::resolve_head(repo_path).expect("Failed to resolve HEAD");

    // 如果目标分支就是当前提交,不需要合并
    if current_commit == target_commit {
        return;
    }

    // 检查冲突
    let current_files = get_commit_files(repo_path, &current_commit);
    let target_files = get_commit_files(repo_path, &target_commit);

    // 检查文件内容是否有冲突
    for (path, target_content) in &target_files {
        if let Some(current_content) = current_files.get(path) {
            if current_content != target_content {
                panic!("Merge conflict in {}: 1", path);
            }
        }
    }

    // 如果没有冲突，继续合并
    let staging_index = Index::load(repo_path);
    let tree_hash = TreeProcessor::create_tree(repo_path, &staging_index.get_staged_files());
    let message = format!("Merge branch '{}'", branch_name);
    git_commit(repo_path, &message);
}

/// 获取提交中的所有文件及其内容
///
/// # 参数
///
/// * `repo_path` - 仓库路径
/// * `commit_hash` - 提交哈希值
///
/// # 返回值
///
/// 返回一个映射，键为文件路径，值为文件内容
fn get_commit_files(repo_path: &str, commit_hash: &str) -> HashMap<String, String> {
    let mut files = HashMap::new();
    read_tree_files(repo_path, commit_hash, "", &mut files);
    files
}

/// 递归读取树对象中的文件内容
///
/// # 参数
///
/// * `repo_path` - 仓库路径
/// * `hash` - 对象哈希值
/// * `prefix` - 当前路径前缀
/// * `files` - 存储结果的映射
fn read_tree_files(repo_path: &str, hash: &str, prefix: &str, files: &mut HashMap<String, String>) {
    // 读取对象文件
    let object_path = format!("{}/.git/objects/{}/{}", repo_path, &hash[..2], &hash[2..]);
    if let Ok(content) = fs::read(&object_path) {
        let content_str = String::from_utf8_lossy(&content);

        // 找到对象类型和实际内容的分隔符
        if let Some(null_pos) = content_str.find('\0') {
            let header = &content_str[..null_pos];
            let parts: Vec<&str> = header.split(' ').collect();
            let obj_type = parts[0];

            match obj_type {
                "commit" => {
                    // 从提交中获取树哈希
                    if let Some(tree_line) = content_str[null_pos + 1..]
                        .lines()
                        .find(|line| line.starts_with("tree "))
                    {
                        let tree_hash = tree_line[5..].trim();
                        read_tree_files(repo_path, tree_hash, prefix, files);
                    }
                }
                "tree" => {
                    // 解析树对象的内容
                    let tree_content = &content_str[null_pos + 1..];
                    let entries = parse_tree_entries(tree_content);

                    for entry in entries {
                        let path = if prefix.is_empty() {
                            entry.name
                        } else {
                            format!("{}/{}", prefix, entry.name)
                        };

                        match entry.mode.as_str() {
                            "100644" | "100755" => {
                                // 普通文件，读取内容
                                read_blob_content(repo_path, &entry.hash, &path, files);
                            }
                            "40000" => {
                                // 目录，递归读取
                                read_tree_files(repo_path, &entry.hash, &path, files);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

/// 读取 Blob 对象的内容
///
/// # 参数
///
/// * `repo_path` - 仓库路径
/// * `hash` - Blob 对象哈希值
/// * `path` - 文件路径
/// * `files` - 存储结果的映射
fn read_blob_content(repo_path: &str, hash: &str, path: &str, files: &mut HashMap<String, String>) {
    let object_path = format!("{}/.git/objects/{}/{}", repo_path, &hash[..2], &hash[2..]);
    if let Ok(content) = fs::read(&object_path) {
        if let Some(null_pos) = content.iter().position(|&b| b == b'\0') {
            if let Ok(file_content) = String::from_utf8(content[null_pos + 1..].to_vec()) {
                files.insert(path.to_string(), file_content);
            }
        }
    }
}

/// 树对象条目结构体
#[derive(Debug)]
struct TreeEntry {
    /// 文件模式（权限）
    mode: String,
    /// 文件名
    name: String,
    /// 对象哈希值
    hash: String,
}

/// 解析树对象内容为 TreeEntry 列表
///
/// # 参数
///
/// * `content` - 树对象的内容
///
/// # 返回值
///
/// 返回解析出的 TreeEntry 列表
fn parse_tree_entries(content: &str) -> Vec<TreeEntry> {
    let mut entries = Vec::new();
    let mut pos = 0;

    while pos < content.len() {
        // 查找模式结束位置（空格）
        if let Some(space_pos) = content[pos..].find(' ') {
            let mode = content[pos..pos + space_pos].to_string();
            pos += space_pos + 1;

            // 查找文件名结束位置（null字节）
            if let Some(null_pos) = content[pos..].find('\0') {
                let name = content[pos..pos + null_pos].to_string();
                pos += null_pos + 1;

                // 读取20字节的SHA-1哈希
                if pos + 20 <= content.len() {
                    let hash = content[pos..pos + 20]
                        .bytes()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>();
                    pos += 20;

                    entries.push(TreeEntry { mode, name, hash });
                }
            }
        } else {
            break;
        }
    }

    entries
}
