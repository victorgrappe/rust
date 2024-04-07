# rust-tuto

The Rust Programming Language tutorial

# Open docs

```bash
rustup docs --book

NEW="02.GuessingGame"; mkdir "./${NEW}"; cp -R "NN.N.Name/" "${NEW}"

git reset; git add .; git status



```

# Rustc

```bash
rustc main.rs
./main
```

# Cargo

```bash
cargo --version

cargo new hello_cargo
cd hello_cargo

cargo check

cargo build
cargo build --release

cargo run

cargo doc --open
```

# Installation

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

```txt
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /Users/victorgrappe/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  /Users/victorgrappe/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /Users/victorgrappe/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /Users/victorgrappe/.profile
  /Users/victorgrappe/.zshenv

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: aarch64-apple-darwin
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
>
```

```bash
vim "${HOME}/.zshenv"
vim "${HOME}/.zprofile"
vim "${HOME}/.zshrc"
vim "${HOME}/.zlogin"

vim "${HOME}/.bash_env"
vim "${HOME}/.bash_profile"
vim "${HOME}/.bashrc"
vim "${HOME}/.bash_login"


vim "${HOME}/.profile"
vim "${HOME}/.cargo/env"


. "${HOME}/.zshenv"
```

```bash
rustc --version
rustup update
```
