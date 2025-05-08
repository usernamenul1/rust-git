use crate::core::reference::Reference;
use crate::core::repository::Repository;

pub fn git_branch(repo_path: &str, branch_name: &str, delete: bool) {
    // 1. 获取仓库对象
    let isrepo = Repository::is_git_repo(repo_path);
    if !isrepo {
        eprintln!("Error: Not a valid git repository at {}", repo_path);
        return;
    }

    if delete {
        // 1. 检查分支是否存在
        if !std::path::Path::new(&format!("{}/.git/refs/heads/{}", repo_path, branch_name)).exists()
        {
            eprintln!("错误: 分支 '{}' 不存在", branch_name);
            return;
        }

        // 2. 检查是否为当前分支（不允许删除当前分支）
        if let Some(current_branch) = Reference::resolve(repo_path, "HEAD") {
            // 解析当前分支名称
            let current_branch = current_branch.trim_start_matches("ref: refs/heads/");
            // 检查当前分支是否与要删除的分支相同
            if current_branch == branch_name {
                eprintln!("错误: 不能删除当前分支 '{}'", branch_name);
                return;
            }
        }

        // 3. 执行删除操作
        Reference::delete(&repo_path, &format!("heads/{}", branch_name));

        // 4. 用户反馈
        println!("已删除分支 '{}'", branch_name);
    }
}
