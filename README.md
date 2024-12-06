# Tarkus

## Overview
This project serves as a hands-on exploration of operating system development while learning the Rust programming language. The project's objectives are currently flexible and may evolve over time.

## Features
- Seamless transition from real mode directly to long (64-bit) mode.
- Global Descriptor Table (GDT) and page tables are fully set up, with the first 2 MiB identity-mapped.
- A foundational kernel has been implemented using Rust, offering essential functionality.
- Initial implementation of an ATA interface to facilitate full kernel loading.

## Prerequisites
To modify or experiment with this project, ensure you have the following tools:
- QEMU
- Make
- Rust (nightly build)
- LLVM Clang
- `ld` linker

## How to Use
1. Clone the repository.
2. Run `cargo clean` and `make clean` to ensure a clean build environment.
3. Execute `make run` to boot the OS in QEMU.

**Note:** Pay close attention to the flags used with QEMU; refer to its documentation for additional details.


## Screenshot
![WhatsApp Image 2024-12-06 at 18 45 12_9f8b01e0](https://github.com/user-attachments/assets/e9dfeccd-1fc8-45a7-a3d2-274e1a1ace5a)
