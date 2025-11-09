#!/bin/bash

echo "Testing QEMU exit functionality..."
echo "Expected: Program exits cleanly with code 0"
echo ""

just build hello_world riscv32i-qemu > /dev/null 2>&1
just disasm hello_world riscv32i-qemu > /dev/null 2>&1

echo "Running hello_world on QEMU..."
qemu-system-riscv32 \
    -machine virt \
    -cpu rv32 \
    -m 128M \
    -nographic \
    -serial mon:stdio \
    -bios none \
    -kernel target/disasm/qemu/riscv32i/hello_world/image.elf

EXIT_CODE=$?
echo ""
echo "Exit code: $EXIT_CODE"

if [ $EXIT_CODE -eq 0 ]; then
    echo "✓ QEMU exited cleanly!"
else
    echo "✗ QEMU exited with error code $EXIT_CODE"
fi
