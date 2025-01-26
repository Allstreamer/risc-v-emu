#!/usr/bin/env bash

riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o test test.s
riscv64-unknown-elf-objcopy -O binary test test.bin

riscv64-unknown-elf-gcc -S fib.c
riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o fib fib.s
riscv64-unknown-elf-objcopy -O binary fib fib.bin
