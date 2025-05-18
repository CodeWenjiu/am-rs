/* Memory layout of the LM3S6965 microcontroller */
/* 1K = 1 KiBi = 1024 bytes */
MEMORY {
  MEM : ORIGIN = 0x80000000, LENGTH = 0x08000000
}

/* The entry point is the reset handler */
ENTRY(__start__)

SECTIONS {
  .text ORIGIN(MEM) : {
    *(.reset_vector)
    *(.text .text.*)
  } > MEM
}
