use crate::core::repository::Repository;

pub fn git_init(args: Vec<String>) {
    // if args.len() < 2 {
    //     println!("Usage: git init <repository>");
    //     return;
    // }

    // let repo_path = &args[1];
    // if std::fs::create_dir_all(repo_path).is_err() {
    //     println!("Error creating directory: {}", repo_path);
    //     return;
    // }

    // let git_dir = format!("{}/.git", repo_path);
    // if std::fs::create_dir_all(&git_dir).is_err() {
    //     println!("Error creating .git directory: {}", git_dir);
    //     return;
    // }

    let git_dir = if args.len() > 1 {
        args[1].clone()
    } else {
        ".".to_string()
    };

    Repository::init(&git_dir);

    println!("Initialized empty Git repository in {}", git_dir);
}
