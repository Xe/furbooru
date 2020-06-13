# furbooru

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)
![Rust](https://github.com/Xe/furbooru/workflows/Rust/badge.svg)
![MIT License](https://img.shields.io/github/license/Xe/furbooru)
[![Crates.io](https://img.shields.io/crates/v/furbooru)](https://crates.io/crates/furbooru)
[![docs](https://img.shields.io/badge/docs-furbooru-orange)](https://docs.rs/furbooru)

A [Furbooru](https://furbooru.org) and [Derpibooru](https://derpibooru.org) client
written in Rust. The APIs for these two sites are near identical, so this crate
can work with both; however it is optimized for Furbooru. Any time Furbooru diverges
from Derpibooru, this crate will follow the Furbooru changes first.

Usage is simple:

```rust
let user_agent = format!(
  "{}/{} ({}, +{})",
  env!("CARGO_PKG_NAME"),
  env!("CARGO_PKG_VERSION"),
  std::env::var("API_USERNAME").unwrap(),
  env!("CARGO_PKG_REPOSITORY"),
);

let cli = furbooru::Client::new(
  user_agent,
  std::env::var("API_TOKEN").unwrap(),
)?
```

Set the environment variables `API_USERNAME` and `API_TOKEN` to your
Furbooru/Derpibooru username and API token respectively. Adding the username
associated with your bot to each request can help the booru staff when your bot
does unwanted things like violating rate limits.

See the examples for more ideas.
