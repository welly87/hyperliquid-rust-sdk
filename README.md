# hyperliquid-rust-sdk

SDK for Hyperliquid API trading with Rust.

## Usage Examples

See `src/bin` for examples. You can run any example with `cargo run --bin [EXAMPLE]`.

## Installation

`cargo add hyperliquid_rust_sdk`

## Message Bus

The crate provides a lightweight `MessageBus` for communicating over NATS. It
supports fire-and-forget `send`, typed `request`/`reply` using correlation IDs,
`subscribe` for streaming typed messages and `publish` for broadcasts.

## License

This project is licensed under the terms of the `MIT` license. See [LICENSE](LICENSE.md) for more details.

```bibtex
@misc{hyperliquid-rust-sdk,
  author = {Hyperliquid},
  title = {SDK for Hyperliquid API trading with Rust.},
  year = {2024},
  publisher = {GitHub},
  journal = {GitHub repository},
  howpublished = {\url{https://github.com/hyperliquid-dex/hyperliquid-rust-sdk}}
}
```
