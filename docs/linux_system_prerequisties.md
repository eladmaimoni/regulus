# Linux / WSL System Prerequisites

```
sudo apt update
sudo apt install ca-certificates gpg wget ninja-build -y

```
# CMake 

Note that by default apt will install an old version of cmake.
we need to tweak it so it will install version 4.3.0 or above.

[ask ubuntu](https://askubuntu.com/questions/355565/how-do-i-install-the-latest-version-of-cmake-from-the-command-line)


1. add kitware GPG key
```
wget -O - https://apt.kitware.com/keys/kitware-archive-latest.asc 2>/dev/null | gpg --dearmor - | sudo tee /usr/share/keyrings/kitware-archive-keyring.gpg >/dev/null
```

2. Add kitware's repository to your sources list and update
Note that this command is specific for ubuntu 24.04
```
echo 'deb [signed-by=/usr/share/keyrings/kitware-archive-keyring.gpg] https://apt.kitware.com/ubuntu/ noble main' | sudo tee /etc/apt/sources.list.d/kitware.list >/dev/null
sudo apt update
```

3. install
```
sudo apt install cmake
```

4. verify cmake has the desired version

```
cmake --version
```

# vcpkg

```bash
git clone https://github.com/microsoft/vcpkg.git ~/workspace/vcpkg
~/workspace/vcpkg/bootstrap-vcpkg.sh
mkdir -p ~/workspace/vcpkg-binary-cache
```

Add the following to `~/.bashrc`:

```bash
export VCPKG_ROOT=~/workspace/vcpkg
export PATH=$VCPKG_ROOT:$PATH
export VCPKG_DEFAULT_BINARY_CACHE=~/workspace/vcpkg-binary-cache
```

Then reload:

```bash
source ~/.bashrc
```

# Clang 22

```bash
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 22
```

Set clang-22 as the default (optional):

```bash
sudo update-alternatives --install /usr/bin/clang clang /usr/bin/clang-22 100
sudo update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-22 100
```

# Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

To update an existing installation:

```bash
rustup update
```






# Linux Cross Compilation for ARM 

## aarch64 sysroot, headers, and cross-linker
```
sudo apt install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
```

## to run the resulting aarch64 binaries for testing
```
sudo apt install qemu-user
```

## add rust target
```
rustup target add aarch64-unknown-linux-gnu
```


# Testing
This will call a rust code that calls C++ code and calls rust code again, and test the results

- testing native x64 debug build: `cargo test --debug`
- testing native x64 release build: `cargo test --release`
- testing arm64 on x64 machine using qemu emulator using the emulator `cargo test --release --target aarch64-unknown-linux-gnu`
