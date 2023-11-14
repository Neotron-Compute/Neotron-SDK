#!/bin/sh
arm-none-eabi-gcc -mcpu=cortex-m0 -Wl,-T../neotron-cortex-m.ld -o asmhello.elf asmhello.S -nostdlib
