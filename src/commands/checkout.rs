use crate::core::reference::Reference;

pub fn git_checkout(repo_path: &str, target: &str) {
    let target_hash = match Reference::resolve(repo_path, target) {
        Some(hash) => hash,
        None => target.to_string(), // 如果是提交哈希，直接使用
    };

    Reference::update_head_to_commit(repo_path, &target_hash).expect("Failed to update HEAD");

    println!("Switched to branch or commit {}", target);
}
