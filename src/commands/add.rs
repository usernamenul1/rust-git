use crate::core::index::Index;

pub fn git_add(repo_path: &str, file_path: &str) {
    // check if the file exists
    let mut staging_area = Index::load(repo_path);

    // save the index
    staging_area.stage_file(file_path);

    // println!("Added file to staging area: {}", file_path);
}
