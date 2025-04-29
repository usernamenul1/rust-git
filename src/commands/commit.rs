use crate::core::{
    commit::CommitBuilder, index::Index, reference::Reference, repository::Repository,
    tree::TreeProcessor,
};

pub fn git_commit(
    repo_path: &str,      // 仓库根路径
    commit_message: &str, // 提交信息
) {
    // 1. 初始化仓库对象
    let repo = Repository::init(repo_path);
    // 2. 加载当前索引
    let staging_index = Index::load(repo_path);
    // 3. 创建树对象哈希
    let tree_hash = TreeProcessor::create_tree(repo_path, &staging_index.get_staged_files());
    // 4. 获取当前分支的最新提交
    let parent_commit = Reference::resolve(repo_path, "refs/heads/master");
    // 5. 创建新的提交对象
    let commit_hash = CommitBuilder::create_commit(
        repo_path,
        tree_hash,
        parent_commit,
        "Author Name".to_string(),
        commit_message.to_string(),
    );
    // 6. 更新当前分支的引用，指向新的提交
    Reference::create(repo_path, "refs/heads/master", &commit_hash);
    let head_path = format!("{}/.git/HEAD", repo.path);
    if let Ok(content) = std::fs::read_to_string(head_path) {
        if content.starts_with("ref: refs/heads/") {
            // HEAD 指向分支，更新分支引用
            let branch = content.trim_start_matches("ref: refs/heads/").trim();
            Reference::create(&repo.path, &format!("heads/{}", branch), &commit_hash);
        } else {
            // 分离 HEAD 状态，直接更新 HEAD
            Reference::update_head_to_commit(&repo.path, &commit_hash)
                .expect("Failed to update HEAD");
        }
    }
    println!("Created commit: {}", commit_hash);
}
