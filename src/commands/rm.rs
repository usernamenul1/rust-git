use crate::core::index::Index;

pub fn git_rm(args: Vec<String>) {
    // Vec: repo_path: &str, file_path: &str
    // check args length
    // if args.len() < 2 {
    //     println!("Usage: git add <file>");
    //     return;
    // }

    // if we directly parse the args, we can use the first arg
    // we have to check if there are two or more file paths before we call the function
    // the second arg is whether force or not
    // check if the file exists
    let mut staging_area = Index::load(args[0].as_str());

    if args[1] == "force" {
        // remove the file from the working directory
        std::fs::remove_file(args[2].as_str()).expect("Failed to remove file");
    } else {
        // remove the file from the working directory
        std::fs::remove_file(args[1].as_str()).expect("Failed to remove file");
    }

    // save the index
    staging_area.unstage_file(args[1].as_str());

    println!("Added file to staging area: {}", args[1]);
}
