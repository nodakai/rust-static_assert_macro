[![Continuous integration](https://github.com/nodakai/rust-static_assert_macro/workflows/Continuous%20integration/badge.svg)](https://github.com/nodakai/rust-static_assert_macro/actions)
[![Crates.io](https://img.shields.io/crates/v/static_assert_macro.svg)](https://crates.io/crates/static_assert_macro)
[![docs.rs](https://docs.rs/static_assert_macro/badge.svg)](https://docs.rs/static_assert_macro/)
[![license](https://img.shields.io/github/license/nodakai/rust-static_assert_macro.svg)](LICENSE) 

# `static_assert` macro

Cargo.toml:

    [dependencies]
    static_assert_macro = "1"

Your code (Rust 2018):

    use static_assert_macro::static_assert;

    static_assert!(1 < 2);

    fn main() {
        static_assert!(3 < 4);
        static_assert!(10 < 10); // build failure
    }

For Rust 2015, replace `use static_assert_macro::static_assert;` with

    #[macro_use]
    extern crate static_assert_macro;

For more details, visit

- https://docs.rs/static_assert_macro/

## Minimum Supported Rust Version

* `static_assert_macro < 1.1`: sufficiently old Rust such as 1.8 (2016-04-14)
* `static_assert_macro >= 1.1`:
  [Rust 1.37 (2019-08-15)](https://github.com/rust-lang/rust/blob/master/RELEASES.md#language-11)
  which stabilized `underscore_const_names`

## License

This crate is a free software distributed under the [Apache 2.0 license](LICENSE).
