<p align="center">
    <img alt="Inifnity OS" width="200" src="https://raw.githubusercontent.com/Infinity-OS/infinity/master/logo.png">
</p>

**Infinity OS** is an operating system written in Rust, with focus on security, simplicity and intelligence.

[![Travis Build Status](https://travis-ci.org/Infinity-OS/infinity.svg?branch=master)](https://travis-ci.org/Infinity-OS/infinity)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)

## Cloning, Building, and Running

To build the Infinity OS you must have an Unix-Like host, and `nasm`, `grup-mkrescue`, `mformat` (including in `mtools`), `xorriso`, `qemu` and a nighly Rust compiler installed on your computer.

### Manual Setup

```bash
$ cd path/to/your/project/folder/

# HTTP
$ git clone https://github.com/Infinity-OS/infinity.git --origin upstream
# SSH
$ git clone git@github.com:Infinity-OS/infinity.git --origin upstream

$ cd infinity/

# Configure
$ ./configure [toolchain_prefix]

# Install rustup.rs
$ curl https://sh.rustup.rs -sSf | sh

# Set override toolchain to nightly build
$ rustup override set nightly

# For successive builds start here. If this is your first build, just continue

# Build Infinity OS
$ make

# Build the changes and run on QEMU
$ make run

# Build a ISO
$ make iso

# Clean all generated files
$ make clean

# Start QEMU in debug mode
$ make debug

# Start a debug session with Radare2
$ make r2
```
