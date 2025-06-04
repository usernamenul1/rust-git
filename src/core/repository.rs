use std::fs::create_dir;

/// Git 仓库结构体
///
/// 表示一个 Git 仓库实例及其基本信息
pub struct Repository {
    /// 仓库所在的文件系统路径
    pub path: String,
}

impl Repository {
    /// 初始化一个新的 Git 仓库
    ///
    /// 在指定路径创建 Git 仓库所需的所有目录和初始文件
    ///
    /// # 参数
    ///
    /// * `path` - 要初始化仓库的路径
    ///
    /// # 返回值
    ///
    /// 返回已初始化的 Repository 实例
    ///
    /// # Panics
    ///
    /// 当目录或文件创建失败时会触发 panic
    pub fn init(path: &str) -> Self {
        // 创建基本目录
        create_dir(format!("{}/.git", path)).expect("Failed to create .git directory");
        create_dir(format!("{}/.git/objects", path)).expect("Failed to create objects directory");
        create_dir(format!("{}/.git/objects/info", path))
            .expect("Failed to create objects/info directory");
        create_dir(format!("{}/.git/objects/pack", path))
            .expect("Failed to create objects/pack directory");
        create_dir(format!("{}/.git/refs", path)).expect("Failed to create refs directory");
        create_dir(format!("{}/.git/refs/heads", path))
            .expect("Failed to create refs/heads directory");
        create_dir(format!("{}/.git/refs/tags", path))
            .expect("Failed to create refs/tags directory");
        create_dir(format!("{}/.git/refs/remotes", path))
            .expect("Failed to create refs/remotes directory");
        create_dir(format!("{}/.git/hooks", path)).expect("Failed to create hooks directory");
        create_dir(format!("{}/.git/info", path)).expect("Failed to create info directory");
        create_dir(format!("{}/.git/logs", path)).expect("Failed to create logs directory");
        create_dir(format!("{}/.git/logs/refs", path))
            .expect("Failed to create logs/refs directory");

        // 创建初始文件
        std::fs::write(format!("{}/.git/HEAD", path), "ref: refs/heads/master\n")
            .expect("Failed to create HEAD file");
        std::fs::write(format!("{}/.git/description", path), "Unnamed repository\n")
            .expect("Failed to create description file");
        std::fs::write(
            format!("{}/.git/config", path),
            "[core]\n\trepositoryformatversion = 0\n\tfilemode = true\n\tbare = false\n",
        )
        .expect("Failed to create config file");
        std::fs::write(
            format!("{}/.git/info/exclude", path),
            "# git ls-files --others --exclude-from=.git/info/exclude\n",
        )
        .expect("Failed to create exclude file");

        Repository {
            path: path.to_string(),
        }
    }

    /// 检查指定路径是否为有效的 Git 仓库
    ///
    /// 通过验证目录结构判断是否存在 Git 仓库
    ///
    /// # 参数
    ///
    /// * `path` - 要检查的路径
    ///
    /// # 返回值
    ///
    /// 如果是 Git 仓库则返回 `true`，否则返回 `false`
    pub fn is_git_repo(path: &str) -> bool {
        // Check if the .git directory exists
        let git_dir = format!("{}/.git", path);
        if std::path::Path::new(&git_dir).exists() {
            // // Check if the objects directory exists
            // let objects_dir = format!("{}/.git/objects", path);
            // if std::path::Path::new(&objects_dir).exists() {
            //     // Check if the refs directory exists
            //     let refs_dir = format!("{}/.git/refs", path);
            //     if std::path::Path::new(&refs_dir).exists() {
            //         return true;
            //     }
            // }
            return true;
        }
        false
    }

    /// 获取已存在仓库的实例
    ///
    /// # 参数
    ///
    /// * `repopath` - 已存在仓库的路径
    ///
    /// # 返回值
    ///
    /// 返回表示指定仓库的 Repository 实例
    pub fn get_repo(repopath: &str) -> Self {
        Self {
            path: repopath.to_string(),
        }
    }
}
