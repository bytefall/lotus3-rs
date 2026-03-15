ENTRY(main)

MEMORY {
  dos : org = 0x0000, len = 0x10000
}

SECTIONS {
  .text   : { *(.startup) *(.text .text.*) } > dos
  .rodata : { *(.rodata .rodata.*) } > dos

  _data = .;
  .data   : { *(.data .data.*) } > dos
  .bss    : { *(.bss .bss.* COMMON) } > dos

  .stack  : { *(.stack) } > dos

  /DISCARD/ : {
    *(.eh_frame .eh_frame.*)
    *(.eh_frame_hdr)
    *(.gcc_except_table .gcc_except_table.*)
    *(.comment)
  }

  _heap = ALIGN(4);
}
