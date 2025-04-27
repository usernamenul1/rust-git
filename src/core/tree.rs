pub struct TreeProcessor;

use crate::core::object::Object;

impl TreeProcessor {
    pub fn create_tree(repo_path: &str, data: &str) -> String {
        // Create a tree object
        let tree_obj = Object::Tree(data.to_string());

        // Save the tree object and return its hash
        tree_obj.save(repo_path)
    }
    
}