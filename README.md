[![crates.io](https://img.shields.io/crates/v/drone-riscv.svg)](https://crates.io/crates/drone-riscv)
![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# drone-riscv

RISC-V platform crate for Drone, an Embedded Operating System.

## Supported Cores

| Architecture | Core name      | Rust target                    | `riscv_core` config flag |
|--------------|----------------|--------------------------------|--------------------------|
| RV32IMAC     | Bumblebee Core | `riscv32imac-unknown-none-elf` | `bumblebee`              |

Rust target triple and `riscv_core` config flag should be set at the
application level according to this table.

## Documentation

- [Drone Book](https://book.drone-os.com/)
- [API documentation](https://api.drone-os.com/drone-riscv/0.14/)

## Usage

Add the crate to your `Cargo.toml` dependencies:

```toml
[dependencies]
drone-riscv = { version = "0.14.0", features = [...] }
```

Add or extend `std` feature as follows:

```toml
[features]
std = ["drone-riscv/std"]
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
