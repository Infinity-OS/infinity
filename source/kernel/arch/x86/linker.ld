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

    .rodata ALIGN(0x1000) : {
        __rodata_start = .;
        *(.rodata)
        *(.rodata.*)
        . = ALIGN(0x1000);
        __rodata_end = .;
    }

    .data ALIGN(0x1000) : {
        __data_start = .;
        *(.data)
        *(.data.*)
        . = ALIGN(0x1000);
        __data_end = .;
    }

    .bss ALIGN(0x1000) : {
        __bss_start = .;
        *(.bss)
        *(.bss.*)
        *(COMMON)
        . = ALIGN(0x1000);
        __bss_end = .;
    }

    __end = .;

    /DISCARD/ : {
        *(.note.GNU-stack)
        *(.comment*)
    }
}
