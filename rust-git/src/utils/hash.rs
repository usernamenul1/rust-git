pub fn sha1(data: &str) -> String {
    use sha1::{Digest, Sha1};
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}