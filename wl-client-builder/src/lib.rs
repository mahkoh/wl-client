//! This crate provides a builder that generates type-safe wrappers for the [`wl-client`]
//! crate.
//!
//! [`wl-client`]: https://docs.rs/wl-client
//!
//! # Generating code in `build.rs`
//!
//! The simplest way to generate code looks like this
//!
//! ```
//! // build.rs
//! use wl_client_builder::Builder;
//!
//! # fn no_run() {
//! fn main() {
//!     Builder::default().build().unwrap();
//! }
//! # }
//! ```
//!
//! This will
//!
//! 1. load all XML protocols stored in the `wayland-protocols` directory next to the
//!    `Cargo.toml`,
//! 2. generate code into `$OUT_DIR/wayland-protocols/mod.rs`.
//!
//! This code can be used with the `include!` macro:
//!
//! ```ignore
//! mod wayland_protocols {
//!     include!(concat!(env!("OUT_DIR"), "/wayland-protocols/mod.rs"));
//! }
//! ```
//!
//! # Enabling mutable data
//!
//! If you want to pass mutable data to event handlers, you must explicitly enable the
//! `mutable_data` flag:
//!
//! ```
//! # use wl_client_builder::Builder;
//! #
//! # fn no_run() {
//! Builder::default().with_mutable_data(true).build().unwrap();
//! # }
//! ```
//!
//! # Generating code with the CLI
//!
//! Wrappers can be generated ahead of time with the [`wl-client-builder`] application.
//!
//! This might be preferable if you want to check the generated code into your repository
//! or if you cannot use `build.rs`.
//!
//! [`wl-client-builder`]: https://crates.io/crates/wl-client-builder-cli
//!
//! Assuming that you are storing the XML files in a directory called `protocols` and
//! want to generate files into `src/protocols/mod.rs`, you can use the CLI as follows:
//!
//! ```shell
//! $ rm -rf src/protocols
//! $ wl-client-builder --xml-dir protocols src/protocols
//! $ rustfmt src/protocols/mod.rs
//! ```
//!
//! # Generating code with a custom application
//!
//! By default the [`Builder`] assumes that it is being used from `build.rs` and will emit
//! messages for cargo. All of this can be customized.
//!
//! ```
//! # use wl_client_builder::Builder;
//! #
//! # fn no_run() {
//! Builder::default()
//!     // Disable build.rs behavior.
//!     .for_build_rs(false)
//!     // Don't try to load XML files from the `wayland-protocols` directory.
//!     .with_default_dir(false)
//!     // ...
//!     .build().unwrap();
//! # }
//! ```

#![allow(clippy::collapsible_else_if, clippy::len_zero)]

pub use {builder::Builder, error::Error};

mod ast;
mod builder;
mod error;
mod formatter;
mod parser;
