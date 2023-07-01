# deboog

[![crates.io](https://img.shields.io/crates/v/deboog.svg)](https://crates.io/crates/deboog)
[![build status](https://img.shields.io/github/actions/workflow/status/unikmhz/deboog/ci.yml?branch=main&logo=github)](https://github.com/unikmhz/deboog/actions)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue)](#license)
[![codecov](https://codecov.io/github/unikmhz/deboog/coverage.svg?branch=main)](https://codecov.io/gh/unikmhz/deboog)
[![documentation](https://docs.rs/deboog/badge.svg)](https://docs.rs/deboog/)

Derive macro for extended debug formatting.

Currently allows skipping fields and masking field values using various strategies, with more to follow.

## Basic usage

Skip serializing a field:

```rust
use deboog::Deboog;

#[derive(Deboog)]
struct Data {
    shown: i32,
    #[deboog(skip)]
    skipped: i32,
}

assert_eq!(
    format!("{:?}", Data { shown: 123, skipped: 234 }),
    r#"Data { shown: 123 }"#
);
```

Also works with tuple structs:

```rust
use deboog::Deboog;

#[derive(Deboog)]
struct Data(i32, #[deboog(skip)] i32);

assert_eq!(
    format!("{:?}", Data(123, 234)),
    r#"Data(123)"#
);
```

And in enums too:

```rust
use deboog::Deboog;

#[derive(Deboog)]
enum Data {
    One,
    Two {
        shown: i32,
        #[deboog(skip)]
        skipped: i32,
    },
}

assert_eq!(
    format!("{:?}", Data::Two { shown: 123, skipped: 234 }),
    r#"Two { shown: 123 }"#
);
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
