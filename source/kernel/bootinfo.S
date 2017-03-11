/**
 * This file is used to define the sections for the Initium recognize this like
 * a valid kernel.
 */

/** Boot information for the image */
.pushsection ".note.initium.image", "a", @note
    .long 1f - 0f
    .long 3f - 2f
    .long 0

0:  .asciz "INITIUM\0"
1:  .p2align 2
2:  .long 1
    .long (1<<0) | (1<<1)
    .p2align 2
3:  .popsection

/** Boot information for where to load the memory */
.pushsection ".note.initium.load", "a", @note
    .long 1f - 0f
    .long 3f - 2f
    .long 1

0:  .asciz "INITIUM\0"
1:  .p2align 2
2:  .long 0
    .long 0
    .quad 0x1000000
    .quad 0x200000
    .quad 0xFFFFFF8000000000
    .quad 0x0000007F80000000
    .p2align 2
3:  .popsection

/** Map in 8GB initially, after that we will map all the available RAM */
.pushsection ".note.initium.mapping.b0xFFFF800000000000", "a", @note
    .long 1f - 0f
    .long 3f - 2f
    .long 3

0:  .asciz "INITIUM\0"
1:  .p2align 2
2:  .quad 0xFFFF800000000000
    .quad 0
    .quad 0x200000000
    .p2align 2
3:  .popsection
