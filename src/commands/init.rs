use crate::core::repository::Repository;

pub fn git_init(path: &str) {
    Repository::init(path);

    // println!("Initialized empty Git repository in {}", path);
}
