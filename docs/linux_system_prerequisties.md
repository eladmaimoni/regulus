# Linux / WSL System Prerequisites

## Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To update an existing installation:

```bash
rustup update
```

## Clang 20

```bash
sudo apt update && sudo apt install clang
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 20
```

Set clang-20 as the default:

```bash
sudo update-alternatives --install /usr/bin/clang clang /usr/bin/clang-20 100
sudo update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-20 100
```

## vcpkg

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


# Linux Cross Compilation for ARM 

## aarch64 sysroot, headers, and cross-linker
```
sudo apt install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
```

## to run the resulting aarch64 binaries for testing
```
home apt install qemu-user
```

## add rust target
```
rustup target add aarch64-unknown-linux-gnu
```