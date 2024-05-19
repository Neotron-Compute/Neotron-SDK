#!/bin/bash

set -euo pipefail

TARGET=${1:-thumbv6m-none-eabi}

if [ "$TARGET" == "thumbv6m-none-eabi" ]; then
    CPU="cortex-m0plus"
elif [ "$TARGET" == "thumbv7m-none-eabi" ]; then
    CPU="cortex-m3"
elif [ "$TARGET" == "thumbv7em-none-eabi" ]; then
    CPU="cortex-m4"
else
    echo "Unknown target"
    exit 1
fi

arm-none-eabi-gcc \
    -fdata-sections \
    -ffreestanding \
    -ffunction-sections \
    -flto \
    -mcpu=$CPU \
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
