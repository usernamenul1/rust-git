use crate::commands::add::git_add;
use crate::commands::branch::git_branch;
use crate::commands::checkout::git_checkout;
use crate::commands::commit::git_commit;
use crate::commands::fetch::git_fetch;
use crate::commands::init::git_init;
use crate::commands::merge::git_merge;
use crate::commands::pull::git_pull;
use crate::commands::push::git_push;
use crate::commands::rm::git_rm;

use crate::cli::args::Cli;
use crate::cli::args::Commands;
use clap::Parser;

pub fn git_execute() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { path }) => git_init(&path),

        Some(Commands::Add { repo_path, file }) => git_add(&repo_path, &file),

        Some(Commands::Rm {
            repo_path,
            file,
            force,
        }) => git_rm(&repo_path, &file, force),

        Some(Commands::Commit { repo_path, message }) => git_commit(&repo_path, &message),

        Some(Commands::Branch {
            repo_path,
            name,
            delete,
        }) => {
            if delete {
                // 删除分支
                if let Some(branch_name) = name {
                    git_branch(&repo_path, &branch_name, true);
                } else {
                    println!("删除分支时需要指定分支名");
                    std::process::exit(1);
                }
            } else if let Some(branch_name) = name {
                // 创建分支
                git_branch(&repo_path, &branch_name, false);
            } else {
                // 列出所有分支
                git_branch(&repo_path, "", false);
            }
        }

        Some(Commands::Checkout { repo_path, target, new_branch }) => git_checkout(&repo_path, &target, new_branch),

        Some(Commands::Merge { repo_path, branch }) => git_merge(&repo_path, &branch),

        Some(Commands::Fetch { repo_path, remote }) => git_fetch(&repo_path, &remote),

        Some(Commands::Pull {
            repo_path,
            remote,
            branch: _,
        }) => git_pull(&repo_path, &remote),

        Some(Commands::Push {
            repo_path,
            remote,
            branch: _,
        }) => git_push(&repo_path, &remote),

        None => {
            println!("请提供一个命令。使用 --help 查看可用命令。");
            std::process::exit(1);
        }
    }
}
