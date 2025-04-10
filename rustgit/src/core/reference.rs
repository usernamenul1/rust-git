pub struct Reference;

impl Reference {
    pub fn create(repo_path: &str, ref_name: &str, target_hash: &str) {
        panic!("Reference::create is not implemented yet");
    }

    pub fn delete(repo_path: &str, ref_name: &str) {
        panic!("Reference::delete is not implemented yet");
    }

    pub fn resolve(repo_path: &str, ref_name: &str) -> Option<String> {
        panic!("Reference::resolve is not implemented yet");
    }
    
}