//! RISC-V platform crate for Drone, an Embedded Operating System.
//!
//! # Supported Cores
//!
//! | Architecture | Core name      | Rust target                    | `riscv_core` config flag |
//! |--------------|----------------|--------------------------------|--------------------------|
//! | RV32IMAC     | Bumblebee Core | `riscv32imac-unknown-none-elf` | `bumblebee`              |
//!
//! Rust target triple and `riscv_core` config flag should be set at the
//! application level according to this table.
//!
//! # Documentation
//!
//! - [Drone Book](https://book.drone-os.com/)
//! - [API documentation](https://api.drone-os.com/drone-riscv/0.13/)
//!
//! # Usage
//!
//! Add the crate to your `Cargo.toml` dependencies:
//!
//! ```toml
//! [dependencies]
//! drone-riscv = { version = "0.13.0", features = [...] }
//! ```
//!
//! Add or extend `std` feature as follows:
//!
//! ```toml
//! [features]
//! std = ["drone-riscv/std"]
//! ```

#![feature(prelude_import)]
#![feature(unsafe_block_in_unsafe_fn)]
#![warn(missing_docs, unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
