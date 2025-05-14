//! Rust 实现的 Git 版本控制系统
//! 
//! 这个库提供了 Git 版本控制系统的核心功能实现，包括：
//! - 基本的 Git 命令行操作
//! - Git 核心数据结构
//! - 仓库管理功能
//! - 文件系统操作

/// 命令行接口相关模块
/// 
/// 这个模块包含所有与命令行交互相关的功能：
/// - args: 命令行参数解析，处理用户输入的各种 Git 命令和选项
/// - command: 命令行命令的实现，将用户输入转换为具体的 Git 操作
pub mod cli {
    pub mod args;
    pub mod command;
}

/// Git 命令实现模块
/// 
/// 包含所有 Git 核心命令的具体实现：
/// - init: 初始化新的 Git 仓库
/// - add: 将文件添加到暂存区
/// - rm: 从暂存区和工作目录中删除文件
/// - commit: 创建新的提交
/// - branch: 分支管理
/// - checkout: 切换分支或还原文件
/// - merge: 合并分支
/// - fetch: 从远程仓库获取数据
/// - pull: 拉取并合并远程分支
/// - push: 推送本地更改到远程仓库
pub mod commands {
    pub mod init;
    pub mod add;
    pub mod rm;
    pub mod commit;
    pub mod branch;
    pub mod checkout;
    pub mod merge;
    pub mod fetch;
    pub mod pull;
    pub mod push;
}

/// Git 核心功能和数据结构模块
/// 
/// 实现 Git 的核心概念和数据结构：
/// - blob: Git 对象中的文件内容存储
/// - commit: 提交对象的实现，包含提交信息、作者、时间等
/// - object: Git 对象系统的基础实现
/// - tree: 目录结构的树形表示
/// - repository: Git 仓库的核心操作和管理
/// - index: 暂存区（索引）的实现
/// - reference: 引用系统（分支、标签等）的实现
pub mod core {
    pub mod blob;
    pub mod commit;
    pub mod object;
    pub mod tree;
    pub mod repository;
    pub mod index;
    pub mod reference;
}

/// 通用工具函数模块
/// 
/// 提供各种辅助功能：
/// - hash: 实现 Git 的哈希计算功能
/// - fs: 文件系统操作的封装
/// - error: 错误类型和错误处理
pub mod utils {
    pub mod hash;
    pub mod fs;
    pub mod error;
}