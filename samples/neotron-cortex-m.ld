/*
 * Neotron Application linker script for Arm Cortex-M systems.
 *
 * Based on the [Rust Embedded Cortex-M crate](https://github.com/rust-embedded/cortex-m).
 * 
 * Copyright (c) 2016 Jorge Aparicio.
 * Copyright (c) 2023 Jonathan 'theJPster' Pallant and the Neotron Developers.
 */

MEMORY
{
    /*
     * This is the Transient Program Area.
     *
     * This is defined by the Neotron specification for a given platform. On this
     * Cortex-M based platform, it's the start of Cortex-M SRAM, plus 4 KiB, or
     * 0x2000_1000.
     */
    RAM   (rwx) : ORIGIN = 0x20001000, LENGTH = 256K
}

/* # Entry point = what the BIOS calls to start the OS */
ENTRY(app_entry);

/* # Sections */
SECTIONS
{
  /* ### .text */
  .text : ALIGN(4)
  {
    . = ALIGN(4);
    *(.text .text.*);
    . = ALIGN(4);
  }

  /* ### .rodata */
  .rodata : ALIGN(4)
  {
    . = ALIGN(4);
    *(.rodata .rodata.*);
    . = ALIGN(4);
  }

  /* ### .data */
  .data : ALIGN(4)
  {
    . = ALIGN(4);
    *(.data .data.*);
    . = ALIGN(4);
  }

  /* LMA of .data */
  __sidata = LOADADDR(.data);

  /* ### .bss */
  .bss : ALIGN(4)
  {
    . = ALIGN(4);
    *(.bss .bss.*);
    . = ALIGN(4);
  }

  /* ### .uninit */
  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    *(.uninit .uninit.*);
    . = ALIGN(4);
  }

  /* ## .got */
  /* Dynamic relocations are unsupported. This section is only used to detect relocatable code in
     the input files and raise an error if relocatable code is found */
  .got (NOLOAD) :
  {
    KEEP(*(.got .got.*));
  }

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Unused exception related info that only wastes space */
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}
