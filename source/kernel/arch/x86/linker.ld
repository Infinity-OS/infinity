ENTRY(start)
OUTPUT_ARCH("i386:x86-64")
OUTPUT_FORMAT("elf64-x86-64")

SECTIONS {
    . = 1M;

    .text :
    {
        *(.text)
    }
}
