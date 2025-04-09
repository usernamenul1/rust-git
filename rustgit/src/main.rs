use rustgit::rust_git::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args.clone());

    /*git command
       git init
       git add
       git commit
       git branch
       git checkout
       git merge

       // choseable git command
       git fetch
       git pull
       git push
    */

    if args.len() < 2 {
        println!("Please provide a git command.");
    }

    // Parse the command line arguments
    let command = &args[1];
    let mut command_args = Vec::new();
    for arg in &args[2..] {
        command_args.push(arg);
    }
    // Match the command and execute the corresponding function
    match command.as_str() {
        "init" => {
            if args.len() > 2 {
                println!("Too many arguments.");
                return;
            }
            git_init()
        }
        "add" => git_add(command_args),
        "commit" => git_commit(command_args),
        "branch" => git_branch(command_args),
        "checkout" => git_checkout(command_args),
        "merge" => git_merge(command_args),
        "fetch" => git_fetch(command_args),
        "pull" => git_pull(command_args),
        "push" => git_push(command_args),
        _ => println!("Unknown command: {}", command),
    }
}
