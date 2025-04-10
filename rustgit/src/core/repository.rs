use core::panic;

pub struct Repository {
    pub path: String,
    // pub objects: Vec<String>,
    // pub refs: Vec<String>,
    // pub index: Vec<String>,
}

impl Repository {
    pub fn init(path: &str) -> Self {
        panic!("Repository::init is not implemented yet");
        // Repository{path}
    }

    pub fn is_git_repo(path: &str) -> bool {
        panic!("Repository::is_git_repo is not implemented yet");
        // check_dir_exists(".git")
    }
}
