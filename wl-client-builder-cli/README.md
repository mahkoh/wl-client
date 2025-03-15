# wl-client-builder

[![crates.io](https://img.shields.io/crates/v/wl-client-builder-cli.svg)](https://crates.io/crates/wl-client-builder-cli)
![MSRV](https://img.shields.io/crates/msrv/wl-client-builder-cli)

The wl-client-builder binary can be used to convert wayland protocol XML files
into safe protocol implementations for [wl-client].

[wl-client]: https://docs.rs/wl-client

You can install it with

```shell
cargo install --locked wl-client-builder-cli
```

## Help

```
Generate safe protocol wrappers for the `wl-client` crate

Usage: wl-client-builder [OPTIONS] <OUT_DIR>

Arguments:
  <OUT_DIR>
          The directory to generate code into.
          
          The generated code will be available via the `mod.rs` file in this directory.
          
          The directory will be created if it does not already exist. Files that already
          exist in the directory will not be removed but might be overwritten. You should
          remove the directory before generating code if you want to ensure that it only
          contains required files.

Options:
      --xml-file <XML_FILE>
          The path to an XML protocol file

      --xml-dir <XML_DIR>
          The path to a directory containing XML protocol files.
          
          This behaves as if all XML files in this directory (but not any sub-directories)
          had been specified explicitly via `--xml-file` arguments.

      --wl-client-path <WL_CLIENT_PATH>
          The rust module path to the `wl-client` crate.
          
          By default, the generated code assumes that the crate can be accessed via
          `::wl_client`. If your crate imports the crate under a different name, the path
          can be modified with this parameter.

  -h, --help
          Print help (see a summary with '-h')
```

## MSRV

The MSRV is `max(1.85, stable - 3)`.

## License

This project is licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.
