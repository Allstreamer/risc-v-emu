#!/usr/bin/env bash

./riscv-gnu-toolchain/bin/riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o test test.s
./riscv-gnu-toolchain/bin/riscv64-unknown-elf-objcopy -O binary test test.bin
