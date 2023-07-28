#!/bin/bash

#
# Builds all the Neotron SDK sample binaries.
#
# Specify the target as the first argument. Defaults to "thumbv6m-none-eabi" if
# not given.
#
# ```console
# $ ./build.sh thumbv7em-none-eabi
# $ ls *.elf
# ```
#

set -euo pipefail

TARGET=${1:-thumbv6m-none-eabi}

mkdir -p ./release

echo "Building for ${TARGET}"
for program in panic hello fault input-test; do
    ( cd ${program} && cargo build --target=${TARGET} --release )
    cp ./${program}/target/${TARGET}/release/${program} ./release/${program}.elf
done
