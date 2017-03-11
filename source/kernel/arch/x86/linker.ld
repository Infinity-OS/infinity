ENTRY(start)
OUTPUT_ARCH("i386:x86-64")
OUTPUT_FORMAT("elf64-x86-64")

SECTIONS {

    /* This define the Kernel's virtual base */
    . = 0xffffffff80000000;
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

    /* Align the start of each segment to a large page boundary to allows
     * the kernel to be mapped with large pages.
     */

    . = ALIGN(0x200000);
    __data_seg_start = .;

    .data : {
        __data_start = .;
        *(.data)
        *(.data.*)
        . = ALIGN(0x1000);
        __data_end = .;
    }

    . = ALIGN(ALIGN(0x1000));

    .bss : {
        __bss_start = .;
        *(.bss)
        *(.bss.*)
        *(COMMON)
        . = ALIGN(0x1000);
        __bss_end = .;
    }

    __data_seg_end = .;

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
