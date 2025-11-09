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

  /DISCARD/ : {
    *(.eh_frame)
    *(.eh_frame_hdr)
  }
}
