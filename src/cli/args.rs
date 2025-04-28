pub fn git_parse_args() -> (&'static str, Option<Vec<String>>) {
    // // 创建一个新的命令行应用
    // 创建命令行应用("rust-git")
    // .设置版本("0.1.0")
    // .设置作者("Your Name <your.email@example.com>")
    // .设置描述("A simple Git implementation in Rust")
    // 。。。
    // // 定义子命令：拉取数据
    // .添加子命令(
    // 创建子命令("fetch")
    // .设置描述("Download objects and refs from another repository")
    // .添加参数("remote_url", "Remote repository URL", 必选)
    // )
    // // 定义子命令：拉取并合并
    // .添加子命令(
    // 创建子命令("pull")
    // .设置描述("Fetch from and integrate with another repository or a
    // local branch")
    // )
    // .添加参数("remote_url", "Remote repository URL", 必选)
    // // 定义子命令：推送更改
    // .添加子命令(
    // 创建子命令("push")
    // .设置描述("Update remote refs along with associated objects")
    // .添加参数("remote_url", "Remote repository URL", 必选)
    // )
    // // 解析命令行参数并返回匹配结果
    // 解析命令行参数()

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return ("", None);
    }
    let command = args[1].as_str();
    let sub_args = if args.len() > 2 {
        Some(args[2..].to_vec())
    } else {
        None
    };
    match command {
        "init" => ("init", sub_args),
        "add" => ("add", sub_args),
        "rm" => ("rm", sub_args),
        "commit" => ("commit", sub_args),
        "branch" => ("branch", sub_args),
        "checkout" => ("checkout", sub_args),
        "merge" => ("merge", sub_args),
        "fetch" => ("fetch", sub_args),
        "pull" => ("pull", sub_args),
        "push" => ("push", sub_args),
        _ => ("unknown", None),
    }
}
