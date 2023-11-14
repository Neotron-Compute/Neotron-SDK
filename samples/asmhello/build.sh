#!/bin/sh
arm-none-eabi-gcc \
    -nostartfiles \
    -ffreestanding \
    -mcpu=cortex-m0plus \
    -Wl,-T../neotron-cortex-m.ld \
    -o asmhello.elf \
    asmhello.S \
