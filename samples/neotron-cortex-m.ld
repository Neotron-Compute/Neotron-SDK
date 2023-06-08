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

  /* ### .entry_point */
  .entry_point ORIGIN(RAM) :
  {
      KEEP(*(.entry_point))
  } > RAM

  PROVIDE(_stext = ADDR(.entry_point) + SIZEOF(.entry_point));

  /* ### .text */
  .text _stext :
  {
    *(.text .text.*);
    *(.HardFaultTrampoline);
    *(.HardFault.*);
  }

  /* ### .rodata */
  .rodata : ALIGN(4)
  {
    *(.rodata .rodata.*);

    /* 4-byte align the end (VMA) of this section.
       This is required by LLD to ensure the LMA of the following .data
       section will have the correct alignment. */
    . = ALIGN(4);
  }

  /* ### .data */
  .data : ALIGN(4)
  {
    . = ALIGN(4);
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    __edata = .;
  }

  /* LMA of .data */
  __sidata = LOADADDR(.data);

  /* ### .bss */
  .bss : ALIGN(4)
  {
    . = ALIGN(4);
    __sbss = .;
    *(.bss .bss.*);
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    __ebss = .;
  }

  /* ### .uninit */
  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    *(.uninit .uninit.*);
    . = ALIGN(4);
  }

  /* Place the heap right after `.uninit` */
  . = ALIGN(4);
  __sheap = .;

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
