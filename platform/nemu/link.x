/* Memory layout of the LM3S6965 microcontroller */
/* 1K = 1 KiBi = 1024 bytes */
MEMORY {
  FLASH : ORIGIN = 0x30000000, LENGTH = 0x08000000
  RAM : ORIGIN = 0x80000000, LENGTH = 0x08000000
}

/* The entry point is the reset handler */
ENTRY(__start__)

SECTIONS {
  .text ORIGIN(FLASH) : 
  {
    *(.reset_vector)
    *(.text .text.*)
  } > FLASH

  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

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
  } > RAM AT> FLASH

  _sidata = LOADADDR(.data);
}
