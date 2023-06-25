#!/bin/bash

set -euo pipefail

TARGET=${1:-thumbv6m-none-eabi}

echo "Building for ${TARGET}"
for program in panic hello fault; do
    ( cd ${program} && cargo build --target=${TARGET} --release )
    cp ./${program}/target/${TARGET}/release/${program} ${program}.elf
done
