[![Continuous integration](https://github.com/nodakai/rust-static_assert_macro/workflows/Continuous%20integration/badge.svg)](https://github.com/nodakai/rust-static_assert_macro/actions)
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-3-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->
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

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://github.com/Boscop"><img src="https://avatars1.githubusercontent.com/u/535593?v=4" width="100px;" alt=""/><br /><sub><b>Boscop</b></sub></a><br /><a href="https://github.com/nodakai/rust-static_assert_macro/commits?author=Boscop" title="Code">💻</a></td>
    <td align="center"><a href="https://stbuehler.de"><img src="https://avatars1.githubusercontent.com/u/528446?v=4" width="100px;" alt=""/><br /><sub><b>Stefan Bühler</b></sub></a><br /><a href="https://github.com/nodakai/rust-static_assert_macro/issues?q=author%3Astbuehler" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/BenWiederhake"><img src="https://avatars3.githubusercontent.com/u/2690845?v=4" width="100px;" alt=""/><br /><sub><b>Ben Wiederhake</b></sub></a><br /><a href="https://github.com/nodakai/rust-static_assert_macro/commits?author=BenWiederhake" title="Code">💻</a></td>
  </tr>
</table>

<!-- markdownlint-enable -->
<!-- prettier-ignore-end -->
<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!