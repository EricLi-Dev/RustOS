# RustOS
An Operating System Kernel written in Rust.

# Docs
Documented at [RustOS Blog](https://ericli-dev.github.io/categories/rustos/)

# Setup
## Nightly Rust Compiler
Use the nightly rust compiler with [rustup](https://rustup.rs/) to opt-in to various experimental features.
```bash
rustup override set nightly
```

## QEMU and VcXsrv
### QEMU
QEMU is an open-source machine emulator that lets you run virtual machines on your computer or emulate different hardware. We are using QEMU to quickly run instances of our OS kernel. [Follow these instructions to install](https://www.qemu.org/download/#linux).

```bash
sudo apt-get install qemu-system
```

### VcXsrv
To properly build this project on WSL an X server is needed on the host Windows machine.

An X server is a GUI environment for Unix-based systems. When running QEMU on a virtual machine, it needs to display that GUI somewhere. Since WSL does not natively support GUI applications, an external X server is needed to render the graphical output from the Linux environment.

Install an X server e.g. [VcXsrv](https://sourceforge.net/p/vcxsrv) on Windows and set the DISPLAY variable in your WSL terminal: 
```bash
export DISPLAY=localhost:0.0
```

### Install bootimage crate
Read [why the bootimage crate is needed](https://os.phil-opp.com/minimal-rust-kernel/#creating-a-bootimage).
```bash
cargo install bootimage
rustup component add llvm-tools-preview
cargo bootimage
```

Finally, build and run OS in QEMU
```bash
cargo run
```
