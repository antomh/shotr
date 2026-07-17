# 'shotr' utility

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](LICENSE)

> **Disclaimer:** This project is created for **educational purposes** to understand how system call tracing works under the hood in Linux. It is not intended to replace the original `strace` in production environments.

## About

`shotr` (short for **show trace**) is a lightweight, educational clone of the popular Linux utility `strace`, written in Rust. It uses the `ptrace` system call to intercept and log system calls made by a process.

While the original `strace` is a massive and feature-rich tool, this project focuses on implementing a minimal subset of features to demonstrate the core mechanics of process tracing, context switching, and signal handling in Rust.

## Features

Currently, the tool supports a minimal set of flags to trace processes and gather statistics:

* **Basic Tracing:** Trace system calls of a given command.
* `-f`, `--follow-forks`: Follow child processes created by `fork()`, `vfork()`, and `clone()`.
* `-c`, `--summary-only`: Count time, calls, and errors for each system call and report a summary table on program exit (format closely mimics the original `strace`).

### Current Limitations (By Design)

* **No PID Attachment:** Tracing of already running processes (the `-p` flag in original `strace`) is **not supported**. `shotr` only traces processes it spawns itself.
* **Architecture:** Currently supports **x86_64** only, handling 64-bit system calls.
* **Argument Decoding:** Advanced decoding of syscall arguments (like string pointers) is minimal or not implemented yet.

## Installation

### Prerequisites

* Rust toolchain (`cargo`)
* Linux OS (uses `ptrace` API)

### Build from source

```bash
git clone https://github.com/antomh/shotr.git
cd shotr
cargo build --release
```

The binary will be located at `target/release/shotr`.

## Usage

**Basic tracing:**

```bash
./shotr ls -l /tmp
```

**Follow forks (e.g., tracing a shell script or a process that spawns children):**

```bash
./shotr -f bash -c "ls && echo 'done'"
```

**Generate a summary table of syscalls (like `strace -c`):**

```bash
./shotr -c ls -l /tmp
```

**Combine flags:**

```bash
./shotr -c -f bash -c "ls && echo 'done'"
```

### A note on permissions

Like the original `strace`, this tool requires permission to attach to processes via `ptrace`.
Depending on your Linux distribution's `kernel.yama.ptrace_scope` settings, you might need to run the tool with `sudo` or adjust the ptrace scope:

```bash
# To allow tracing by any process (temporary, until reboot)
echo 0 | sudo tee /proc/sys/kernel/yama/ptrace_scope
```

## Architecture & Design Choices

If you are looking at this project for educational purposes, here are some technical details:

1. Zero `clap` Dependency: The command-line argument parsing is implemented from scratch without using clap or similar crates, to better understand argument parsing logic.

2. [`nix`](https://crates.io/crates/nix?spm=a2ty_o01.29997173.0.0.4e0e55fbKQ3mCi) used for safe Rust wrappers around Linux system calls (ptrace, waitpid, etc.).

## License

This project is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**. 

This is a strong copyleft license: you are free to use, modify, and distribute this software, but any derivative works or network interactions based on it **must also be open-sourced** under the exact same license.

See the [LICENSE](LICENSE.md) file for the exact legal text.
