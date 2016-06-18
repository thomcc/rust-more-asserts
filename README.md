# More Asserts

Small library providing some macros helpful for asserting.

## Documentation

Use this crate by adding a `#[macro_use] extern crate more_asserts` to your code
after adding it as a dependency.

The following macros are provided.

- `assert_ne!(left, right)`: Panics if `!(left != right)`.
- `assert_lt!(left, right)`: Panics if `!(left < right)`.
- `assert_gt!(left, right)`: Panics if `!(left > right)`.
- `assert_le!(left, right)`: Panics if `!(left <= right)`.
- `assert_ge!(left, right)`: Panics if `!(left >= right)`.
- `debug_assert_ne!(left, right)`: Variant of `assert_ne!` controlled by `cfg!(debug_assertions)`.
- `debug_assert_lt!(left, right)`: Variant of `assert_lt!` controlled by `cfg!(debug_assertions)`.
- `debug_assert_gt!(left, right)`: Variant of `assert_gt!` controlled by `cfg!(debug_assertions)`.
- `debug_assert_le!(left, right)`: Variant of `assert_le!` controlled by `cfg!(debug_assertions)`.
- `debug_assert_ge!(left, right)`: Variant of `assert_ge!` controlled by `cfg!(debug_assertions)`.
- `debug_unreachable!(...)`: Variant of the standard library's `unreachable!`
  that is controlled by `cfg!(debug_assertations)`. Usful mainly when reaching the
  code is a bug that you absolutely want to know about, but there's a sane way to
  proceed in production code regardless. Forwards any arguments passed to it to
  `unreachable!`.

Note that `assert_eq!` and `debug_assert_eq!` are not provided as those are in the standard library.

## License

[CC0 (public domain)](https://creativecommons.org/publicdomain/zero/1.0/).


