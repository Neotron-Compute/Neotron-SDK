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
    -nostartfiles \
    -ffreestanding \
    -mcpu=$CPU \
    -Wl,-T../../neotron-cortex-m.ld \
    -o asmhello.elf \
    asmhello.S \
