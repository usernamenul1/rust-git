use crate::core::index::Index;

pub fn git_rm(repo_path: &str, file_path: &str, force_mode: bool) {
    let mut staging_area = Index::load(repo_path);

    if force_mode {
        // remove the file from the working directory
        std::fs::remove_file(file_path).expect("Failed to remove file");
    } else {
        // remove the file from the working directory
        std::fs::remove_file(file_path).expect("Failed to remove file");
    }

    // save the index
    staging_area.unstage_file(file_path);

    // println!("Removed file from staging area: {}", file_path);
}
