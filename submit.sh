cargo build --release
rm rust-git.zip
zip -r rust-git.zip src target/release/rust-git Cargo.toml Cargo.lock
