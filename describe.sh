#!/bin/bash

# Copyright (c) Ferrous Systems, 2023
#
# This work is licensed under a Creative Commons Attribution-ShareAlike 4.0 International License.

set -euo pipefail

GIVEN_REF=$1

case "${GIVEN_REF}" in
  refs/heads/*)
    slug="$(git branch --show)-$(git rev-parse --short HEAD)"
    ;;
  refs/tags/*)
     slug="$(echo "${GIVEN_REF}" | awk '{split($0,a,"/"); print a[3]}')"
     ;;
  refs/pull/*/merge)
     slug="pr-$(echo "${GIVEN_REF}" | awk '{split($0,a,"/"); print a[3]}')-$(git rev-parse --short HEAD)"
     ;;
esac

echo "${slug}"
