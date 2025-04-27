pub struct BlobProcessor;

use crate::core::object::Object;

impl BlobProcessor {
    pub fn create_blob(repo_path: &str, data: &str) -> String {
        // // Create a blob object
        // let blob_content = format!("blob {}\0{}", data.len(), data);
        let blob_obj = Object::Blob(data.to_string());

        // Save the blob object and return its hash
        blob_obj.save(repo_path)
        // panic!("BlobProcessor::create_blob is not implemented yet");
    }
}
