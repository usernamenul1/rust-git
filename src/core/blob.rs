pub struct BlobProcessor;

use crate::core::object::Object;

impl BlobProcessor {
    /// 创建一个新的 Blob 对象
    ///
    /// # 参数
    ///
    /// * `repo_path` - 仓库路径
    /// * `data` - 要存储在 Blob 中的数据
    ///
    /// # 返回值
    ///
    /// 返回新创建的 Blob 对象的 SHA-1 哈希值
    pub fn create_blob(repo_path: &str, data: &str) -> String {
        // // Create a blob object
        // let blob_content = format!("blob {}\0{}", data.len(), data);
        let blob_obj = Object::Blob(data.to_string());

        // Save the blob object and return its hash
        blob_obj.save(repo_path)
        // panic!("BlobProcessor::create_blob is not implemented yet");
    }
}
