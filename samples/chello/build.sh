#!/bin/sh
arm-none-eabi-gcc -Os -mcpu=cortex-m0 -Wl,-T../neotron-cortex-m.ld -o chello.elf chello.c -nostdlib
