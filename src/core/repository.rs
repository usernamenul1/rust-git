use std::fs::create_dir;

pub struct Repository {
    pub path: String,
}

impl Repository {
    pub fn init(path: &str) -> Self {
        // 创建基本目录
        create_dir(format!("{}/.git", path)).expect("Failed to create .git directory");
        create_dir(format!("{}/.git/objects", path)).expect("Failed to create objects directory");
        create_dir(format!("{}/.git/objects/info", path)).expect("Failed to create objects/info directory");
        create_dir(format!("{}/.git/objects/pack", path)).expect("Failed to create objects/pack directory");
        create_dir(format!("{}/.git/refs", path)).expect("Failed to create refs directory");
        create_dir(format!("{}/.git/refs/heads", path)).expect("Failed to create refs/heads directory");
        create_dir(format!("{}/.git/refs/tags", path)).expect("Failed to create refs/tags directory");
        create_dir(format!("{}/.git/refs/remotes", path)).expect("Failed to create refs/remotes directory");
        create_dir(format!("{}/.git/hooks", path)).expect("Failed to create hooks directory");
        create_dir(format!("{}/.git/info", path)).expect("Failed to create info directory");
        create_dir(format!("{}/.git/logs", path)).expect("Failed to create logs directory");
        create_dir(format!("{}/.git/logs/refs", path)).expect("Failed to create logs/refs directory");
        
        // 创建初始文件
        std::fs::write(format!("{}/.git/HEAD", path), "ref: refs/heads/main\n")
            .expect("Failed to create HEAD file");
        std::fs::write(format!("{}/.git/description", path), "Unnamed repository\n")
            .expect("Failed to create description file");
        std::fs::write(format!("{}/.git/config", path), "[core]\n\trepositoryformatversion = 0\n\tfilemode = true\n\tbare = false\n")
            .expect("Failed to create config file");
        std::fs::write(format!("{}/.git/info/exclude", path), "# git ls-files --others --exclude-from=.git/info/exclude\n")
            .expect("Failed to create exclude file");
        
        Repository {
            path: path.to_string(),
        }
    }

    pub fn is_git_repo(path: &str) -> bool {
        // Check if the .git directory exists
        let git_dir = format!("{}/.git", path);
        if std::path::Path::new(&git_dir).exists() {
            // Check if the objects directory exists
            let objects_dir = format!("{}/.git/objects", path);
            if std::path::Path::new(&objects_dir).exists() {
                // Check if the refs directory exists
                let refs_dir = format!("{}/.git/refs", path);
                if std::path::Path::new(&refs_dir).exists() {
                    return true;
                }
            }
        }
        false
    }
}
