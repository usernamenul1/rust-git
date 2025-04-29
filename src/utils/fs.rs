pub fn create_dir(path: &str) {
    if !std::path::Path::new(path).exists() {
        std::fs::create_dir_all(path).expect("Failed to create directory");
    }
}

pub fn write_file(path: &str, data: String) {
    let mut file = std::fs::File::create(path).expect("Failed to create file");
    use std::io::Write;
    file.write_all(data.as_bytes())
        .expect("Failed to write to file");
}

pub fn read_file(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
