ENTRY(start)
OUTPUT_ARCH("i386:x86-64")
OUTPUT_FORMAT("elf64-x86-64")

SECTIONS {
    . = 0xffffffff80000000 + SIZEOF_HEADERS;

    .text : {
        __text_start = .;
        *(.text)
        *(.text.*)
        . = ALIGN(0x1000);
        __text_end = .;
    }

    .notes : {
        *(.note.*)
    }

    /DISCARD/ : {
        *(.note.GNU-stack)
        *(.comment*)
    }
}
