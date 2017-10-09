<p align="center">
    <img alt="Inifnity OS" width="200" src="https://raw.githubusercontent.com/Infinity-OS/infinity/master/logo.png">
</p>

**Infinity OS** is an operating system written in Rust, with focus on security, simplicity and intelligence.

[![Travis Build Status](https://travis-ci.org/Infinity-OS/infinity.svg?branch=master)](https://travis-ci.org/Infinity-OS/infinity)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)

## Cloning, Building, and Running

Before you start running and mess around with this awesome project you must prepare your system. First of all, to build Infinity OS you need to have an Unix-Like host. Everything else depends on your preferred method to do that.

Down below, you can thing different ways of prepare or system. We recommend using docker, you that is up to you.

### Cloning

The code bellow is used to clone the project, as well download all required Git modules. Put it on a well-chosen place, and try not move it! 😉

```bash
$ cd path/to/your/project/folder/

# HTTP
$ git clone https://github.com/Infinity-OS/infinity.git --origin upstream
# SSH
$ git clone git@github.com:Infinity-OS/infinity.git --origin upstream

$ cd infinity/
```

### Method 1: Using Docker

There are innumerous advantages in compiling Infinity OS using  Docker. When you use Docker, you don't need to prepare your system for compiling the OS, our image already contains everything that you need for. The only thing that you need installed on your host is Qemu and Docker, of course.

```bash
# Build the image
$ make d_init

# Run make command on the container
$ make d_make

# Start an interactive session
$ make d_inter
```

### Method 2: Manual Setup

Using this method you need to install the following packages on your system: `nasm`, `grup-mkrescue`, `mformat` (including in `mtools`), `xorriso`, `qemu` and a nighly Rust compiler installed on your computer.

Please make sure you use the latest nightly of rustc before building. Otherwise, the build process can result in an error.

```bash
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

> Note: When using a MacOS it's needed a cross compiler to use the LD command. If you don't want do that you can try the first method, using Docker.
