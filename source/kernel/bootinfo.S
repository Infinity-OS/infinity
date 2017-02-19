/**
 * This file is used to define the sections for the Initium recognize this like
 * a valid kernel.
 */

.pushsection ".note.initium.image", "a", @note
.long 1f - 0f
.long 3f - 2f
.long 0
0: .asciz "INITIUM\0"
1: .p2align 2
2: .long 1
   .long (1<<0) | (1<<1)
   .p2align 2
3: .popsection
