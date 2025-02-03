# RustOS
An Operating System Kernel written in Rust.

# Docs
Documented at [RustOS Blog](https://ericli-dev.github.io/categories/rustos/)

# Setup
Use the nightly rust compiler with [rustup](https://rustup.rs/) to opt-in to various experimental features.
```
rustup override set nightly
```

If building on WSL, install an X server e.g. [VcXsrv](https://sourceforge.net/p/vcxsrv) on Windows and set the DISPLAY variable in your WSL terminal: `export DISPLAY=localhost:0.0`
