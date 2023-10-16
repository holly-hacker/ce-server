# `ce-server`

This repo houses a Windows server for Cheat Engine. It allows you to remotely use some Cheat Engine functionality without running the entire Cheat Engine on the host machine.

## Usage

1. Launch this server
2. In Cheat Engine, open the Process List window and click "Network" at the bottom.
3. Enter the host name/IP and port and click "Connect".
4. In the Process List window, use the Windows tab to select a process.

## Features

Currently, this server supports basic Cheat Engine functionality such as reading, writing and scanning memory.

Here is an incomplete list of what it can do:

- Enumerate processes, modules and memory regions
- Read/write/scan memory

Here is an incomplete list of what it can **not** do:

- Debug
- Use the speed hack
- Create, pause or resume threads
- Allocate or deallocate memory
- Use extensions
- Execute AOB scans
- Make you a sandwich

## Building

Install the Rust toolchain (see [rustup](https://rustup.rs/)) and run `cargo build --release`, which will build a release binary in the `target/release` folder.
