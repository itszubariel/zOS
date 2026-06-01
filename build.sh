#!/bin/bash

set -e

build_all() {
    nasm -f elf64 boot/boot.s -o boot/boot.o
    cargo build --release
    ld -n -o isodir/boot/zos.bin -T linker.ld boot/boot.o target/x86_64-unknown-none/release/libzos.a
    grub-mkrescue -o zos.iso isodir
}

case "$1" in
    all)
        build_all
        qemu-system-x86_64 -cdrom zos.iso
        ;;
    build)
        build_all
        ;;
    run)
        qemu-system-x86_64 -cdrom zos.iso
        ;;
    asm)
        nasm -f elf64 boot/boot.s -o boot/boot.o
        ;;
    *)
        echo "Usage: $0 [all|build|run|asm]"
        ;;
esac