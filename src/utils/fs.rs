/// 创建目录
///
/// 如果目录不存在，则创建指定的目录（包括所有必要的父目录）
///
/// # 参数
///
/// * `path` - 要创建的目录路径
///
/// # Panics
///
/// 当目录创建失败时会触发 panic
pub fn create_dir(path: &str) {
    if !std::path::Path::new(path).exists() {
        std::fs::create_dir_all(path).expect("Failed to create directory");
    }
}

/// 创建文件
///
/// 如果文件不存在，则创建一个空文件
///
/// # 参数
///
/// * `path` - 要创建的文件路径
///
/// # Panics
///
/// 当文件创建失败时会触发 panic
pub fn create_file(path: &str) {
    if !std::path::Path::new(path).exists() {
        std::fs::File::create(path).expect("Failed to create file");
    }
}

/// 写入文件内容
///
/// 创建文件并写入指定的数据
///
/// # 参数
///
/// * `path` - 目标文件路径
/// * `data` - 要写入的内容
///
/// # Panics
///
/// 当文件创建或写入失败时会触发 panic
pub fn write_file(path: &str, data: String) {
    let mut file = std::fs::File::create(path).expect("Failed to create file");
    use std::io::Write;
    file.write_all(data.as_bytes())
        .expect("Failed to write to file");
}

/// 读取文件内容
///
/// 读取指定文件的全部内容并以字符串形式返回
///
/// # 参数
///
/// * `path` - 要读取的文件路径
///
/// # 返回值
///
/// 返回 `Result<String, std::io::Error>`，成功时包含文件内容，失败时包含 IO 错误
pub fn read_file(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
