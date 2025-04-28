use crate::core::index::Index;

pub fn git_add(args: Vec<String>) {
    // Vec: repo_path: &str, file_path: &str
    // check args length
    // if args.len() < 2 {
    //     println!("Usage: git add <file>");
    //     return;
    // }

    // if we directly parse the args, we can use the first arg
    // we have to check if there are two or more file paths before we call the function

    // check if the file exists
    let mut staging_area = Index::load(args[0].as_str());

    // save the index
    staging_area.stage_file(args[1].as_str());

    println!("Added file to staging area: {}", args[1]);
}
