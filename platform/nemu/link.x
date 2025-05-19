/* Memory layout of the LM3S6965 microcontroller */
/* 1K = 1 KiBi = 1024 bytes */
MEMORY {
  RAM : ORIGIN = 0x80000000, LENGTH = 0x08000000
}

/* The entry point is the reset handler */
ENTRY(.reset_vector)

SECTIONS {
  .text ORIGIN(RAM) : 
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
