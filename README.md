# Rust std library on ESP32 C3 RISC-V boards playground

In this project I am playing with ESP32 C3 RISC-V based board, with the `std` library (another option would be using the `no-std` - bare-metal -, but I discarded it because I want to be as close to higher level application development as possible).

## Environment Setup

I started my journey from the official [Rust on ESP Book](https://esp-rs.github.io/book/introduction.html).

In case you want to go fast without going through the book, try following the next howto steps, which will let you run this project in your ESP32 C3 RISC-V based board.

- Rust installation via `rustup`, and required dependencies via `espup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install ldproxy
rustup toolchain install nightly --component rust-src  # since we are targetting RISC-V based boards
cargo install espup
espup install  # IMPORTANT to add to your shell profile the following line: . $HOME/export-esp.sh
cargo install cargo-espflash
cargo install espflash
cargo install cargo-espmonitor
cargo install espmonitor
```

**NOTE**: as already mentioned, and if any issue, please follow the book mentioned above 🫰

- Check that the board is connected and ready to be used. In the new generated repo path, run:

```bash
cargo espflash board-info
```

- Setup the IDE extensions:

Since I am using VSCode I installed the following extensions for [Rust](https://esp-rs.github.io/book/tooling/text-editors-and-ides.html#visual-studio-code).

And, the `CodeLLDB` extension is recommended.

## Develop and Release

Once the base setup is done, make sure to create the `cfg.toml` from the `cfg.toml.template` template file, accordingly. Then:

### Run in debug mode

With or without IDE, one can start developing by building and flashing in debug mode to the selected serial-connected device, and monitor the output on that serial:

```bash
cargo run
```

Here, thanks to using Rust, errors are pretty clear and helpful, if any. Have fun! 🦀

### Run release

Once the development is ready, one can release an optimized binary and flash it with the following command:

```bash
cargo espflash --release
```

If requiring to also check the serial monitor, run instead:

```bash
cargo espflash --release --monitor
```

## Final notes

In any case, I recommend reading the book, since this list of steps above is merely meant as a documentation recap for this repo's context.

After setting up the development environment, I did also generate this base repo via `cargo generate` with the official `esp-idf-template` template:

```bash
cargo install cargo-generate
cargo generate --git https://github.com/esp-rs/esp-idf-template cargo
```

**NOTE**: you will need to give it a `project` name, select the MCU `esp32c3`, set `true` for STD support, set `false` for DevContainers, and select `ESP-IDF` native build version (I did select v4.4, which was the current stable back then).

And, once I got the base setup, then I continued over [Writing std applications](https://esp-rs.github.io/book/writing-your-own-application/std-applications/index.html).

I also based my learnings on the following repositories:

- [rust-esp32-std-demo](https://github.com/ivmarkov/rust-esp32-std-demo)
- [espressif-trainings](https://github.com/esp-rs/espressif-trainings)

This setup is great because you keep using the same main tools Rust provides, and all cross-compiled! 🚀
