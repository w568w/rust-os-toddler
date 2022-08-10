# Rust OS
A simple operating system in Rust.

**Note:** NOT a complete kernel, currently only a **toy OS** for learning and entertainment purposes, with *no* curial features.
## Features
- support `print!()` & `println!()`
## Compile
1. Change the QEMU directory to your own in `image_builder/src/main.rs`.

2. With [Rust nightly enabled](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#rustup-and-the-role-of-rust-nightly), execute this command to build and run the OS in the emulator:
    ```shell
    $ cargo xrun
    ```
