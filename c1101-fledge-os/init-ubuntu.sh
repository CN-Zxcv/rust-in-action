sudo apt install -y qemu-system-x86
sudo apt install -y seabios

# 支持直接操作可执行文件
cargo install cargo-binutils
# 支持构建启动映像
cargo install bootimage

# 切换到 nightly 版本，我们需要使用一些还未稳定的功能
rustup toolchain install nightly
rustup default nightly

# rust 源码，为新系统编译一个编译器
# rustup component add rust-src
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

# LLVM 编译器
rustup component add llvm-tools-preview
