[![Build Status](https://travis-ci.org/nodakai/rust-static_assert_macro.svg?branch=master)](https://travis-ci.org/nodakai/rust-static_assert_macro)
[![Crates.io](https://img.shields.io/crates/v/static_assert_macro.svg)](https://crates.io/crates/static_assert_macro)
[![license](https://img.shields.io/github/license/nodakai/rust-static_assert_macro.svg)](https://github.com/nodakai/rust-static_assert_macro/blob/master/LICENSE) 

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

For Rust 2015 code, replace `use static_assert_macro::static_assert;` with

    #[macro_use]
    extern crate static_assert_macro;

For more details, visit

- http://nodakai.github.io/rust-static_assert_macro/static_assert_macro/

This crate is a free software distributed under the Apache 2.0 license.
