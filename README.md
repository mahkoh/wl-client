# wl-client

[![crates.io](https://img.shields.io/crates/v/wl-client.svg)](https://crates.io/crates/wl-client)
[![docs.rs](https://docs.rs/wl-client/badge.svg)](https://docs.rs/wl-client)
![MSRV](https://img.shields.io/crates/msrv/wl-client)

The wl-client crate provides a safe wrapper around libwayland. It should be used
together with [wl-client-builder] to generate safe protocol wrappers.

[wl-client-builder]: https://docs.rs/wl-client-builder

## Safety

libwayland is a mostly thread safe, reentrant, callback-based library. wl-client
contains a large amount of unsafe code to make this not only safe but also
convenient, e.g. by supporting scoped event handlers. wl-client contains ~200
unit tests that were created via manual mutation testing to achieve a nearly
100% test coverage. All of these tests are run through miri.

## MSRV

The MSRV is `max(1.85, stable - 3)`.

## License

This project is licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.
