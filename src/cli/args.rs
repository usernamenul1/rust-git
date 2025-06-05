use clap::{Parser, Subcommand};

/// Git 命令行解析主结构体
///
/// 用于解析命令行参数，定义应用的基本信息
#[derive(Parser)]
#[command(name = "rust-git")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(version = "0.1.0")]
#[command(about = "A simple Git implementation in Rust", long_about = None)]
pub struct Cli {
    /// 子命令枚举，表示要执行的 Git 命令
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Git 子命令枚举
///
/// 定义所有支持的 Git 命令及其参数
#[derive(Subcommand)]
pub enum Commands {
    /// 初始化一个新的仓库
    Init {
        /// 仓库路径，默认为当前仓库
        #[arg(default_value = ".")]
        path: String,
    },
    /// 向仓库中添加文件
    Add {
        /// 待添加的文件
        file: String,
        /// 仓库路径
        #[arg(hide = true, default_value = ".")]
        repo_path: String,
    },
    /// 从仓库中移除文件
    Rm {
        /// 待移除的文件
        file: String,

        /// 强制移除，非必选，默认为false
        #[arg(hide = true, long, default_value_t = false)]
        force: bool,
        /// 仓库路径
        #[arg(hide = true, default_value = ".")]
        repo_path: String,
    },
    /// commit，提交更改
    Commit {
        /// [-m|--message 提交更改信息] 必选
        #[arg(short = 'm', long = "message", required = true)]
        message: String,
        /// 仓库路径
        #[arg(default_value = ".")]
        repo_path: String,
    },
    /// 列出/创建/删除分支
    Branch {
        /// 分支名
        name: Option<String>,
        /// [-d|--delete] 删除分支
        #[arg(short = 'd', long = "delete")]
        delete: bool,
        /// 仓库路径
        #[arg(default_value = ".")]
        repo_path: String,
    },
    /// 切换分支
    Checkout {
        /// 待切换的分支名或commit
        #[arg(required = true)]
        target: String,
        /// 仓库路径
        #[arg(default_value = ".")]
        repo_path: String,
        // checkout -b
        /// [-b|--branch] 创建新的分支并切换到它
        #[arg(short = 'b', long = "branch")]
        new_branch: bool,
    },
    /// 合并两个分支
    Merge {
        /// 待合并的分支
        branch: String,
        /// 仓库路径
        #[arg(default_value = ".")]
        repo_path: String,
    },
    /// 从另一个仓库下载对象和引用
    Fetch {
        /// 仓库路径
        #[arg(default_value = ".")]
        repo_path: String,
        /// 远程仓库地址
        remote: String,
    },
    /// 把远程仓库拉取到本地仓库
    Pull {
        /// 仓库路径
        #[arg(default_value = ".")]
        repo_path: String,
        /// 远程仓库URL
        remote: String,
        /// 待拉取的分支，默认为main
        #[arg(default_value = "main")]
        branch: String,
    },
    /// 把本地仓库推送到远程仓库
    Push {
        /// 本地仓库路径
        #[arg(default_value = ".")]
        repo_path: String,
        /// 远程仓库URL
        remote: String,
        /// 待推送的分支，默认为main
        #[arg(default_value = "main")]
        branch: String,
    },
}

/// 解析 Git 命令行参数
///
/// # 返回值
///
/// 返回一个元组，包含命令名称和可选的参数列表
pub fn git_parse_args() -> (&'static str, Option<Vec<String>>) {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { path }) => ("init", Some(vec![path])),
        Some(Commands::Add { file, repo_path }) => ("add", Some(vec![repo_path, file])),
        Some(Commands::Rm {
            file,
            repo_path,
            force,
        }) => {
            let mut args = vec![repo_path, file];
            if force {
                args.insert(1, "force".to_string());
            }
            ("rm", Some(args))
        }
        Some(Commands::Commit { repo_path, message }) => ("commit", Some(vec![repo_path, message])),
        Some(Commands::Branch {
            repo_path,
            name,
            delete,
        }) => {
            let mut args = vec![repo_path];

            // 处理 delete 标志
            if delete {
                args.push("--delete".to_string());
            }

            // 处理分支名称
            if let Some(branch_name) = name {
                args.push(branch_name);
            }

            ("branch", Some(args))
        }
        Some(Commands::Checkout {
            repo_path,
            target,
            new_branch,
        }) => (
            "checkout",
            Some(vec![repo_path, target, new_branch.to_string()]),
        ),
        Some(Commands::Merge { repo_path, branch }) => ("merge", Some(vec![repo_path, branch])),
        Some(Commands::Fetch { repo_path, remote }) => ("fetch", Some(vec![repo_path, remote])),
        Some(Commands::Pull {
            repo_path,
            remote,
            branch,
        }) => ("pull", Some(vec![repo_path, remote, branch])),
        Some(Commands::Push {
            repo_path,
            remote,
            branch,
        }) => ("push", Some(vec![repo_path, remote, branch])),
        None => ("", None),
    }
}
