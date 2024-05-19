/*
 * Neotron Application linker script for Arm Cortex-M systems.
 *
 * Based on the [Rust Embedded Cortex-M crate](https://github.com/rust-embedded/cortex-m).
 * 
 * Copyright (c) 2016 Jorge Aparicio.
 * Copyright (c) 2024 Jonathan 'theJPster' Pallant and the Neotron Developers.
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
EXTERN(app_entry);
ENTRY(app_entry);

/* # Sections */
SECTIONS
{
  /* ## .text */
  /* All the executable code for our program */
  .text : ALIGN(4)
  {
    . = ALIGN(4);
    *(.text .text.*);
    . = ALIGN(4);
  } > RAM

  /* ## .rodata */
  /* All the read-only static data for our program */
  .rodata : ALIGN(4)
  {
    . = ALIGN(4);
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } > RAM

  /* ## .data */
  /* All the read-write non-zero-initialised static data for our program */
  .data : ALIGN(4)
  {
    . = ALIGN(4);
    *(.data .data.*);
    . = ALIGN(4);
  } > RAM

  /* ## .bss */
  /* All the read-write zero-initialised static data for our program */
  .bss : ALIGN(4)
  {
    . = ALIGN(4);
    *(.bss .bss.*);
    . = ALIGN(4);
  } > RAM

  /* ## .uninit */
  /* All the read-write uninitialised static data for our program */
  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    *(.uninit .uninit.*);
    . = ALIGN(4);
  } > RAM

  /* ## Start of Heap */
  /* Newlib's `sbrk` syscall uses the `end` symbol to mark the start of heap
     memory. The `sbrk` syscall has a static variable tracking the edge of the
     heap (`heap_end`) and that value moves upwards as more heap memory is
     required. The `heap_end` value is initialised with the address of the `end`
     symbol on first allocation.

     See https://github.com/bminor/newlib/blob/master/libgloss/libnosys/sbrk.c
     */
  . = ALIGN(4);
  end = .;

  /* ## .got */
  /* Dynamic relocations are unsupported. This section is only used to detect
     relocatable code in the input files and raise an error if relocatable code
     is found */
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
