use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rust-git")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(version = "0.1.0")]
#[command(about = "A simple Git implementation in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new repository
    Init {
        /// Path to the repository
        #[arg(default_value = ".")]
        path: String,
    },
    /// Add file contents to the index
    Add {
        /// File to add
        file: String,
        /// Repository path
        #[arg(hide = true, default_value = ".")]
        repo_path: String,
        
    },
    /// Remove files from the working tree and the index
    Rm {
        /// File to remove
        file: String,
               
        /// Force removal
        #[arg(hide = true, long, default_value_t = false)]
        force: bool,
        /// Repository path
        #[arg(hide = true, default_value = ".")]
        repo_path: String,
    },
    /// Record changes to the repository
    Commit {
        
        /// Commit message
        #[arg(short = 'm', long = "message", required = true)]
        message: String,
        /// Repository path
        #[arg(default_value = ".")]
        repo_path: String,
    },
    /// List, create, or delete branches
    Branch {
        /// Branch name
        name: Option<String>,
        /// Delete branch
        #[arg(short = 'd', long = "delete")]
        delete: bool,
        /// Repository path
        #[arg(default_value = ".")]
        repo_path: String,
    },
    /// Switch branches or restore working tree files
    Checkout {
        /// Branch or commit to checkout
        #[arg(required = true)]        
        target: String,
        /// Repository path
        #[arg(default_value = ".")]
        repo_path: String,
    },
    /// Join two or more development histories together
    Merge {
        /// Branch to merge
        branch: String,
        /// Repository path
        #[arg(default_value = ".")]
        repo_path: String,
    },
    /// Download objects and refs from another repository
    Fetch {
        /// Repository path
        #[arg(default_value = ".")]
        repo_path: String,
        /// Remote repository URL
        remote: String,
    },
    /// Fetch from and integrate with another repository or a local branch
    Pull {
        /// Repository path
        #[arg(default_value = ".")]
        repo_path: String,
        /// Remote repository URL
        remote: String,
        /// Branch to pull
        #[arg(default_value = "main")]
        branch: String,
    },
    /// Update remote refs along with associated objects
    Push {
        /// Repository path
        #[arg(default_value = ".")]
        repo_path: String,
        /// Remote repository URL
        remote: String,
        /// Branch to push
        #[arg(default_value = "main")]
        branch: String,
    },
}

pub fn git_parse_args() -> (&'static str, Option<Vec<String>>) {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { path }) => ("init", Some(vec![path])),
        Some(Commands::Add { file,repo_path  }) => ("add", Some(vec![repo_path, file])),
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
        Some(Commands::Checkout { repo_path, target }) => {
            ("checkout", Some(vec![repo_path, target]))
        }
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
