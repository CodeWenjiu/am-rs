MEMORY {
  RAM : ORIGIN = 0x80000000, LENGTH = 0x08000000
}

ENTRY(_start)

SECTIONS {
  . = ORIGIN(RAM);

  .text : {
    KEEP(*(.text._start))
    KEEP(*(.text.__start__))
    *(.text .text.*)
  } > RAM

  .rodata : {
    *(.rodata .rodata.*)
  } > RAM

  .data : {
    *(.data .data.*)
  } > RAM

  .bss : {
    *(.bss .bss.*)
    *(COMMON)
  } > RAM

  . = ALIGN(4);
  _sdata = ADDR(.data);
  _edata = ADDR(.data) + SIZEOF(.data);
  _sbss = ADDR(.bss);
  _ebss = ADDR(.bss) + SIZEOF(.bss);

  /* Heap region - starts after BSS and extends to end of RAM */
  . = ALIGN(4);
  _sheap = .;
  _eheap = ORIGIN(RAM) + LENGTH(RAM) - 0x100000;  /* Reserve 1MB for stack */

  /* Stack region - at the top of RAM */
  _stack_top = ORIGIN(RAM) + LENGTH(RAM);

  /DISCARD/ : {
    *(.eh_frame)
    *(.eh_frame_hdr)
  }
}
