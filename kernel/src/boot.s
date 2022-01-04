.global start

/* Starting in 32 bit mode */
.code32

/* Define a multiboot header in it's own section so we can start the binary with it. */

.set ALIGN,    1<<0             /* align loaded modules on page boundaries */
.set MEMINFO,  1<<1             /* provide memory map */
.set FLAGS,    ALIGN | MEMINFO  /* this is the Multiboot 'flag' field */
.set MAGIC,    0x1BADB002       /* 'magic number' lets bootloader find the header */
.set CHECKSUM, -(MAGIC + FLAGS) /* checksum of above, to prove we are multiboot */

.section .multiboot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

/* Allocate some space for a small stack (16 byte aligned) */
.section .bss.kernel
.align 16
stack_bottom:
.skip 16384 # 16 KiB
stack_top:

.section .data.kernel
/* Setup static GDT with flat code and data segments for 64bit */
.set GDT_SIZE, 3 * 8
gdt:
gdt_null:
	.word 0xffff
	.word 0
	.byte 0
	.byte 0
	.byte 1
	.byte 0
gdt_code:
    .word 0
    .word 0
    .byte 0
    .byte 0b10011010
    .byte 0b10101111
    .byte 0
gdt_data:
    .word 0
    .word 0
    .byte 0
    .byte 0b10010010
    .byte 0b00000000
    .byte 0
gdt_pointer:
	.word GDT_SIZE - 1
	.quad gdt

boot_data:
mb_magic:
    .long 0
mb_data_ptr:
    .quad 0
    .quad KERNEL_START
    .quad KERNEL_END

/*
Actual boot script. Since this is a multiboot kernel, we start in 32bit mode. Starting for that,
we will:
    - Setup 4 Level paging for the first two MiBs of memory
    - Enable long mode
    - Load 64bit GDT and jump into full long mode
    - Setup the stack
    - Jump into rust code
*/
.section .init
start:
    /* preserve multiboot data */
    mov %eax, (mb_magic)
    mov %ebx, (mb_data_ptr)
setup_pages:
    /* Clear the memory from 0x1000 to 0x4FFF */
    mov $0x1000, %edi           # Set 0x1000 in destination register
    mov %edi, %cr3              # Set page directory pointer
    mov $0, %eax                # Empty EAX for stosd
    mov $4096, %ecx             # Set rep counter to 4096 (l = 4 bytes, so the size of four tables)
    rep stosl                   # Clear memory

    /* Set up page forwarding all the way down with present and r/w bits set */
    movl $0x2003, (0x1000)
    movl $0x3003, (0x2000)
    movl $0x4003, (0x3000)

    /* Setup page table for first two MiBs (one table) */
    mov $0x4000, %edi           # Base of page table
    mov $3, %ebx                # 0 index page with bits present and r/w
    mov $512, %ecx              # 512 entries in loop counter
set_entry:
    mov %ebx, (%edi)            # Write current table entry
    add $0x1000, %ebx           # Add one page offset to current EBX
    add $8, %edi                # Set memory address for next page entry
    loop (set_entry)            # Go back to write entry

    /* Enable PAE (bit 5 of cr4) */
    mov %cr4, %eax
    or $(1 << 5), %eax
    mov %eax, %cr4

    /* Enable LM (bit 8 of EFER) */
    mov $0xC0000080, %ecx
    rdmsr
    or $(1 << 8), %eax
    wrmsr

    /* Enable PG (bit 31 of CR0) */
    mov %cr0, %eax
    or $(1 << 31), %eax
    mov %eax, %cr0

    lgdt (gdt_pointer)          # Load new GDT
    jmp $0x8, $start64          # Jump into full long mode

/* Start of 64bit code */
.code64
start64:
    /* Point all data segments to new segment. */
    mov $0x10, %ax
    mov %ax, %ds
    mov %ax, %es
    mov %ax, %fs
    mov %ax, %gs
    mov %ax, %ss

    /* initialise stack */
    mov $stack_top, %rsp

    /* rust call argument */
    mov $boot_data, %rdi

	/*
    Now that we are in long mode and have a well defined stack, we can
    move into rust code.
    Note that we already pushed the multiboot info onto the stack at the very beginning.
	*/
	call kernel_main

	/*
	If the system has nothing more to do, put the computer into an
	infinite loop. To do that:
	1) Disable interrupts with cli (clear interrupt enable in eflags).
	   They are already disabled by the bootloader, so this is not needed.
	   Mind that you might later enable interrupts and return from
	   kernel_main (which is sort of nonsensical to do).
	2) Wait for the next interrupt to arrive with hlt (halt instruction).
	   Since they are disabled, this will lock up the computer.
	3) Jump to the hlt instruction if it ever wakes up due to a
	   non-maskable interrupt occurring or due to system management mode.
	*/
	cli
1:	hlt
	jmp 1b
