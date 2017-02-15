# infinity

Infinity OS is an operative system written in Rust, with focus on security, simplicity and intelligence.

## Building

To Build the Infinity OS you must have an Unix-Like host, the [Scons](http://scons.org) installed on your computer and run the following commands:

```bash
# Update git submodules
$ git submodule update --recursive --init

# Configure
$ scons config

# Build the Toolchain
$ scons toolchain

# Build Infinity OS
$ scons

# Build the changes and run on Qemu
$ scons qemu
```
