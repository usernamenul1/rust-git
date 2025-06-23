cp ../target/release/rust-git .

# 执行 ./test/下的所有测试
for test_file in ./tests/test*.sh; do
    # 执行测试文件
    bash "$test_file"
    # 检查上一个命令的返回值
    if [ $? -ne 0 ]; then
        echo "Test failed: $test_file"
        exit 1
    fi
done
echo "All tests passed."