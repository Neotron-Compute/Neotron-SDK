#!/bin/bash

#
# Builds all the Neotron SDK sample binaries.
#
# Specify the target as the first argument. Defaults to "thumbv6m-none-eabi" if
# not given.
#
# ```console
# $ ./build.sh thumbv7em-none-eabi
# $ ls ./release/*.elf
# ```
#

set -euo pipefail

TARGET=${1:-thumbv6m-none-eabi}

mkdir -p ./release

echo "Building for host"
cargo build

echo "Building for ${TARGET}"
cargo build --target ${TARGET} --release

for program in panic hello fault input-test; do
    cp ./target/${TARGET}/release/${program} ./release/${program}.elf
done
