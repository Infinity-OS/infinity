# infinity

Infinity OS is an operative system written in Rust, with focus on security, simplicity and intelligence.

## Building

To build the Infinity OS you must have an Unix-Like host, and `nasm`, `grup-mkrescue`, `mformat` (including in `mtools`), `xorriso`, `qemu` and a nighly Rust compiler installed on your computer.

```bash
# Configure
$ ./configure [toolchain_prefix]

# Build Infinity OS
$ make

# Build the changes and run on Qemu
$ make run

# Build a ISO
$ make iso

# Clean all generated files
$ make clean
```
