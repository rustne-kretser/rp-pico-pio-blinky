# Blinky for RP Pico med PIO

Denne koden hører sammen med [dette blogg-innlegget](https://blog.rustnekretser.no/rust-pa-rp-pico/).

## Installasjon

For å kjøre dette må du først installere [`rustup` og
Rust](https://www.rust-lang.org/tools/install). I tillegg må du
installere noen ekstra pakker:

```sh
rustup target install thumbv6m-none-eabi
cargo install flip-link
cargo install elf2uf2-rs --locked
```

## Kjøre eksempelet på RP pico

```sh
cargo run
```
