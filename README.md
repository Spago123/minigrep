## Minigrep is a simple CLI tool written in RUST. To run the tests navigate to minigrip_lib and run:
```
cargo test
```

## To generate a test report in the same directory type:

```
cargo +nightly test -- --format=json -Z unstable-options --report-time > test-report.json
```

## Before you run this you must install nightly toolchain:
 - for Windows:
 ```
    cargo install nightly-x86_64-pc-windows-msvc
```
 - for Linux:
 ```
 rustup toolchain install nightly-x86_64-unknown-linux-gnu
```

## To run the CLI tool navigate to minigrip and run: 
```
cargo run -- <text_to_search> <path_to_file> > <output_file>
```