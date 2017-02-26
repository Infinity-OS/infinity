ENTRY(start)
OUTPUT_ARCH("i386:x86-64")
OUTPUT_FORMAT("elf64-x86-64")

SECTIONS {

    /* This define the Kernel's virtual base */
    . = 1M;
    . += SIZEOF_HEADERS;
    . = ALIGN(0x1000);

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

    /* Discard some unnecessary sections */
    /DISCARD/ : {
        *(.note.GNU-stack)
        *(.note.gnu.*)
        *(.comment*)
        *(.debug*)
        *(.eh_frame*)
        *(.gcc_except_table*)
        *(.rel.eh_frame*)
    }
}
