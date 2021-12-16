ENTRY(start)

SECTIONS
{
	/* Begin putting sections at 1 MiB so we have 14 MiBs of memory available. */
	. = 0x00100000;
	KERNEL_START = .;

	/* We're the multiboot header and all boot sections at the very beginning of memory. This
	 is only necessary for the multiboot header, but this makes it easier to debug the early boot
	 process. */
	.init : ALIGN(4K)
	{
		*(.multiboot)
		*(.init)
		*(.bss.kernel)
		*(.data.kernel)
    }

    .text : ALIGN(4K)
    {
		*(.text .text.*)
	}

	/* Read-only data. */
	.rodata : ALIGN(4K)
	{
		*(.rodata .rodata.*)
	}

	/* Read-write data (initialized) */
	.data : ALIGN(4K)
	{
		*(.data .data.*)
	}

	/* Read-write data (uninitialized) and stack */
	.bss : ALIGN(4K)
	{
		*(COMMON)
		*(.bss .bss.*)
		KERNEL_END = .;
	}
}
