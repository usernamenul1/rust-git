use crate::cli::args::git_parse_args;
use crate::commands::add::git_add;
use crate::commands::branch::git_branch;
use crate::commands::checkout::git_checkout;
use crate::commands::commit::git_commit;
use crate::commands::fetch::git_fetch;
use crate::commands::init::git_init;
use crate::commands::merge::git_merge;
use crate::commands::pull::git_pull;
use crate::commands::push::git_push;
use crate::commands::rm::git_rm;

pub fn git_execute() {
    let command = git_parse_args();
    match command {
        ("init", Some(args)) => git_init(args),
        ("add", Some(args)) => git_add(args),
        ("rm", Some(args)) => git_rm(args),
        ("commit", Some(args)) => git_commit(args),
        ("branch", Some(args)) => git_branch(args),
        ("checkout", Some(args)) => git_checkout(args),
        ("merge", Some(args)) => git_merge(args),
        ("fetch", Some(args)) => git_fetch(args),
        ("pull", Some(args)) => git_pull(args),
        ("push", Some(args)) => git_push(args),
        _ => {
            println!("Unknown command");
            std::process::exit(1);
        }
    }
}
