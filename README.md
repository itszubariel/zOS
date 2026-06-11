# zOS

A hobbyist x86_64 kernel built in Rust.

## Prerequisites
- Git
- Rust (`rustup` recommended)
- NASM
- QEMU
- Binutils (`ld`)
- GRUB/Xorriso

## Installation

### Linux (Arch/Debian/Ubuntu)
```bash
# Arch
sudo pacman -S nasm qemu-full grub xorriso

# Debian/Ubuntu
sudo apt install nasm qemu-system-x86 grub-pc-bin xorriso
```

### MacOS
```bash
# Using Homebrew
brew install nasm qemu xorriso
```

### Windows
Use WSL2 and follow the Linux instructions.

```bash
# Setup
rustup target add x86_64-unknown-none
```

## Build and Run
The `build.sh` script automates the compilation and ISO creation process:

```bash
# Build, create ISO, and run in QEMU
./build.sh all

# Just build
./build.sh build

# Run last built ISO
./build.sh run
```

## Features
- Full-screen scrollable terminal with 100-line history.
- Color preservation for all terminal output.
- Manual scrolling with Page Up/Down keys.
- Dynamic theme switching (red/blue).
- Keyboard input with Shift/Caps Lock/Backspace.
- Command processing (help, clear, echo, version, uptime, sysinfo, theme).
- Clean module-based architecture.
