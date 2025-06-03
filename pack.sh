#!/bin/bash

# Set the output zip file name
ZIP_NAME="rust-git.zip"

cd ..

# Navigate to the parent directory of rust-git
cd "$(dirname "$0")" || exit 1

# Check if the rust-git directory exists
if [ ! -d "rust-git" ]; then
  echo "Error: rust-git directory not found!"
  exit 1
fi

# Create the zip file with the specified files and directories
zip -r "$ZIP_NAME" \
  rust-git/src \
  rust-git/target/release/rust-git \
  rust-git/Cargo.toml \
  rust-git/Cargo.lock

echo "Successfully created $ZIP_NAME"

