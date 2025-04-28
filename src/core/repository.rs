use std::fs::create_dir;

pub struct Repository {
    pub path: String,
    // pub objects: Vec<String>,
    // pub refs: Vec<String>,
    // pub index: Vec<String>,
}

impl Repository {
    pub fn init(path: &str) -> Self {
        // panic!("Repository::init is not implemented yet");
        // create_dir(path).expect("Failed to create directory");
        create_dir(format!("{}/.git", path)).expect("Failed to create .git directory");
        create_dir(format!("{}/.git/objects", path)).expect("Failed to create objects directory");
        create_dir(format!("{}/.git/refs", path)).expect("Failed to create refs directory");
        create_dir(format!("{}/.git/hooks", path)).expect("Failed to create hooks file");
        create_dir(format!("{}/.git/info", path)).expect("Failed to create info file");
        create_dir(format!("{}/.git/logs", path)).expect("Failed to create logs directory");
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

    // 获取当前分支的最新提交
    pub fn get_current_branch(&self) -> String {
        // panic!("Repository::get_current_branch is not implemented yet");
        let head_file = format!("{}/.git/HEAD", self.path);
        if std::path::Path::new(&head_file).exists() {
            let contents = std::fs::read_to_string(head_file).expect("Failed to read HEAD file");
            if contents.starts_with("ref: ") {
                let branch = contents[5..].trim();
                return branch.to_string();
            }
        }
        "master".to_string()
    }

    // 获取当前分支的最新提交
    pub fn get_latest_commit(&self, branch: &str) -> String {
        // panic!("Repository::get_latest_commit is not implemented yet");
        let ref_file = format!("{}/.git/refs/{}", self.path, branch);
        if std::path::Path::new(&ref_file).exists() {
            let contents = std::fs::read_to_string(ref_file).expect("Failed to read reference file");
            return contents.trim().to_string();
        }
        // If the reference file does not exist, return an empty string
        "".to_string()
    }
}
