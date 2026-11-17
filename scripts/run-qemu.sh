#!/usr/bin/env bash
# Usage (via cargo): scripts/run-qemu.sh <kernel-elf> [extra qemu args...]
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
kernel="$1"; shift

cp "$kernel" "$root/esp/boot/kernel"

exec qemu-system-x86_64 \
  -M q35 -m 256M \
  -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE_4M.fd \
  -drive format=raw,file=fat:rw:"$root/esp" \
  -serial stdio \
  -no-reboot \
  "$@"
