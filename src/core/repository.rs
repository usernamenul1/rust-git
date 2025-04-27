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
        create_dir(format!("{}/.git/HEAD", path)).expect("Failed to create HEAD file");
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
