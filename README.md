# [More Asserts](https://crates.io/crates/more-asserts) (for Rust).

[![Docs](https://docs.rs/more-asserts/badge.svg)](https://docs.rs/more-asserts)
[![Latest Version](https://img.shields.io/crates/v/arcstr.svg)](https://crates.io/crates/more-asserts)
![Minimum Rust Version](https://img.shields.io/badge/MSRV%201.46.0-blue.svg)

Small library providing assertion macros similar to the `{debug_,}assert_{eq,ne}` macros in the stdlib.

## Usage

```rust
use more_asserts as ma;

#[derive(Debug, PartialEq, PartialOrd)]
enum Example { Foo, Bar }

ma::assert_le!(3, 4);
ma::assert_ge!(
    10, 10,
    "You can pass a message too (just like `assert_eq!`)",
);
ma::debug_assert_lt!(
    1.3, 4.5,
    "Format syntax is supported ({}).",
    "also like `assert_eq!`"
);

ma::assert_gt!(
    Example::Bar, Example::Foo,
    "It works on anything that implements PartialOrd and Debug!",
);
```

## License

[CC0 (public domain)](https://creativecommons.org/publicdomain/zero/1.0/).
