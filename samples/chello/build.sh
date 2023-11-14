#!/bin/sh

arm-none-eabi-gcc \
    -fdata-sections \
    -ffreestanding \
    -ffunction-sections \
    -flto \
    -mcpu=cortex-m0 \
    -nostartfiles \
    -Os \
    -Wall \
    -Wconversion \
    -Wdouble-promotion \
    -Wextra \
    -Wl,-gc-sections \
    -Wl,-T../neotron-cortex-m.ld \
    -Wshadow \
    --specs=nano.specs \
    --specs=nosys.specs \
    -o chello.elf \
    stubs.c \
    chello.c
