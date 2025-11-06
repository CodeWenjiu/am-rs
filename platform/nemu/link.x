MEMORY {
  RAM : ORIGIN = 0x80000000, LENGTH = 0x08000000
}

ENTRY(__start__)

SECTIONS {
  . = ORIGIN(RAM);

  .text :
  {
    *(.reset_vector)
    *(.text .text.*)
  } > RAM

  .rodata :
  {
    *(.rodata .rodata.*);
  } > RAM

  .bss :
  {
    _sbss = .;
    *(.bss .bss.*);
    _ebss = .;
  } > RAM

  .data :
  {
    _sdata = .;
    *(.data .data.*);
    _edata = .;
  } > RAM
}
