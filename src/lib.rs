//! A library implementation of so-called `static_assert` based on a macro which
//! conditionally expands into an integer overflow, breaking the compilation.
//!
//! It can take one or more constant boolean expressions as its parameters.
//! When you have multiple conditions to verify, pass them to a single
//! invocation of this macro as multiple parameters.
//!
//! **You cannot invoke this macro more than once in a given scope.**
//! This is because its implementation is based on a type synonym definition and
//! two or more invocation causes name collision.
//!
//! ## Example
//! ```
//! #[macro_use]
//! extern crate static_assert_macro;
//!
//! static_assert!(0 < 1, 1 < 2, 2 < 3);
//!
//! fn main() {
//!     const FOUR: i32 = 4;
//!     static_assert!(1 + 2 == { FOUR - 1 });
//! }
//! ```

#[macro_export]
macro_rules! static_assert {
    (let $e:expr; ) => (
        type _ArrayForStaticAssert = [i8; ($e) as usize - 1];
    );

    (let $e:expr; $e1:expr $(, $ee:expr)*) => (
        static_assert!(let ($e) && ($e1); $($ee),*);
    );

    ($e:expr $(, $ee:expr)*) => (
        static_assert!(let true && ($e); $($ee),*);
    );
}

#[cfg(test)]
mod tests {
    const _FOUR: i32 = 4;

    #[test]
    fn test_0() {
        static_assert!(1 + 2 == { _FOUR - 1 }, 2 < 3);
    }
    #[test]
    fn test_1() {
        static_assert!(true);
    }
    #[test]
    fn test_2() {
        // static_assert!(1); // should not compile
    }
}
