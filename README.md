# 🌐 pg_idna

[IDNA](https://crates.io/crates/idna) helpers for PostgreSQL.

[![Rust](https://img.shields.io/badge/Language-Rust-blue?style=flat-square)](https://www.rust-lang.org)
[![Build Status](https://github.com/owenthewizard/pg_idna/actions/workflows/rust_ci.yml/badge.svg?style=flat-square)](https://github.com/owenthewizard/pg_idna/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-green?style=flat-square)](https://opensource.org/licenses/MIT)
[![License: Apache](https://img.shields.io/badge/License-Apache-green?style=flat-square)](https://opensource.org/licenses/Apache-2-0)
[![Crates.io](https://img.shields.io/crates/v/pg_idna?style=flat-square)](https://crates.io/crates/pg_idna)
[![Contributors](https://img.shields.io/github/contributors/owenthewizard/pg_idna?style=flat-square)](https://github.com/owenthewizard/pg_idna/graphs/contributors)
[![GitHub forks](https://img.shields.io/github/forks/owenthewizard/pg_idna?style=flat-square)](https://github.com/owenthewizard/pg_idna/network/members)
[![Stars](https://img.shields.io/github/stars/owenthewizard/pg_idna?style=flat-square)](https://github.com/owenthewizard/pg_idna/stargazers)
[![Issues](https://img.shields.io/github/issues/owenthewizard/pg_idna?style=flat-square)](https://github.com/owenthewizard/pg_idna/issues)

## ▶️ Quick Start

### 🏗️ Build & Run

```sh
git clone --depth=1 https://github.com/owenthewizard/pg_idna.git && cd pg_idna
cargo pgrx run --release
```

### 🧑‍💻 Have Fun!

```sql
CREATE EXTENSION pg_idna;
SELECT idna_to_ascii('☕.us');
"xn--53h.us"
SELECT idna_to_unicode('xn--53h.us');
"☕.us"
```

## 🚧 Warning

This project is in a pre-alpha stage. Do not use it in production.
I am not responsible if your elephant explodes.

## 🚀 Performance

Benchmarks are yet to be introduced. In my brief testing, ~1 MM domains could be processed every second.

### 👷 Code Style

Obey `rustfmt` and Rust 2021 conventions, as well as `clippy` lints.

## 🤝 Contributions

Pull requests are always welcome.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed under the terms of both the MIT License and the Apache License (Version 2.0).

## 🔢 Version Scheme

At the moment, this project does not have a stable versioning scheme.

Changes will be documented in the [Changelog](CHANGELOG.md) on a best-effort basis.

See the [tags](https://github.com/owenthewizard/pg_idna/tags) for available releases.

## 👪 Authors

See [the list of contributors](https://github.com/owenthewizard/pg_inda/contributors).

## ⚖️ License

See LICENSE-APACHE and LICENSE-MIT for details.

## 🫶 Acknowledgements

- [rust-url/idna](https://crates.io/crates/idna) by [The Servo Project Developers](https://servo.org/)
- [pgrx](https://crates.io/crates/pgrx) by Various Authors
