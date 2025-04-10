use std::collections::HashSet;

pub struct Index {
    repo_path: String,
    staged_files: HashSet<String>,
}

impl Index {
    pub fn load(repo_path: &str) -> Self {
        let index_file = format!("{}/.git/index", repo_path);

        // Check if the index file exists
        if !std::path::Path::new(&index_file).exists() {
            // If it doesn't exist, create a new index
            std::fs::File::create(&index_file).expect("Failed to create index file");
        } else {
            // If it exists, load the index from the file
            let contents = std::fs::read_to_string(&index_file).expect("Failed to read index file");
            let staged_files: HashSet<String> = contents.lines().map(|s| s.to_string()).collect();
            return Index {
                repo_path: repo_path.to_string(),
                staged_files,
            };
        }
        panic!("Index::load is implemented by artificial intelligence");
        Index {
            repo_path: repo_path.to_string(),
            staged_files: HashSet::new(),
        }
    }

    pub fn stage_file(&mut self, file_path: &str) {
        // Add the file to the staged files set

        // Write the staged files to the index file

        panic!("not implemented yet");
    }

    pub fn unstage_file(&mut self, file_path: &str) {
        // Remove the file from the staged files set

        // Write the staged files to the index file

        panic!("not implemented yet");
    }

    fn persist(&self) {
        // Write the staged files to the index file
        panic!("not implemented yet");
    }
}
