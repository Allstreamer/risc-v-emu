#!/usr/bin/env bash

riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o test test.s
riscv64-unknown-elf-objcopy -O binary test test.bin
