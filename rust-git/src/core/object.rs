use crate::utils::fs::{create_dir, write_file};
use crate::utils::hash::sha1;

pub enum Object {
    Blob(String),
    Tree(String),
    Commit(String),
}

impl Object {
    pub fn save(&self, repo_path: &str) -> String {
        let raw_data = match self {
            Object::Blob(data) => format!("blob {}\0{}", data.len(), data),
            Object::Tree(data) => format!("tree {}\0{}", data.len(), data),
            Object::Commit(data) => format!("commit {}\0{}", data.len(), data),
        };

        let hash = sha1(&raw_data);

        let dir = format!("{}/.git/objects/{}", repo_path, &hash[0..2]);
        create_dir(&dir);
        write_file(&format!("{}/{}", repo_path, &hash[2..]), raw_data);
        hash
    }
}
