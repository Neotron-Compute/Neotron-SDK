#!/bin/bash

set -euo pipefail

TARGET=${1:-thumbv6m-none-eabi}

echo "Building for ${TARGET}"
for program in panic hello fault; do
    ( cd ${program} && cargo build --target=${TARGET} --release )
    rust-objcopy -O binary ./${program}/target/${TARGET}/release/${program} ${program}.bin
done
