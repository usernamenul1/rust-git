pub struct Reference;

impl Reference {
    pub fn create(repo_path: &str, ref_name: &str, target_hash: &str) {
        // panic!("Reference::create is not implemented yet");
        let ref_path = format!("{}/.git/refs/{}", repo_path, ref_name);
        let ref_content = format!("ref: {}", target_hash);
        std::fs::write(ref_path, ref_content).expect("Failed to write reference");
    }

    pub fn delete(repo_path: &str, ref_name: &str) {
        let ref_path = format!("{}/.git/refs/{}", repo_path, ref_name);
        std::fs::remove_file(ref_path).expect("Failed to delete reference");
    }

    pub fn resolve(repo_path: &str, ref_name: &str) -> Option<String> {
        if std::path::Path::new(&format!("{}/.git/refs/{}", repo_path, ref_name)).exists() {
            let ref_content =
                std::fs::read_to_string(format!("{}/.git/refs/{}", repo_path, ref_name))
                    .expect("Failed to read reference");
            return Some(ref_content.trim().to_string());
        }
        None
    }
}
