//! A library implementation of so-called `static_assert` based on a macro which
//! conditionally expands into an integer overflow, breaking the compilation.
//!
//! It can take one or more constant boolean expressions as its parameters.
//! When you have multiple conditions to verify, pass them to a single
//! invocation of this macro as multiple parameters.
//!
//! **You cannot invoke this macro more than once in a given scope.**
//! This is because its implementation is based on a function definition and
//! two or more invocations cause name collision.
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
        fn _fn_for_static_assert(_: [i8; ($e) as usize - 1]) {}
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
    #[test]
    fn test_simple() {
        static_assert!(true);
    }

    #[test]
    fn test_simple_failure() {
        // static_assert!(false); // should not compile
    }

    const _FOUR: i32 = 4;

    #[test]
    fn test_const_arithmetic() {
        static_assert!(1 + 2 == { _FOUR - 1 });
    }

    #[test]
    fn test_multiple_asserts() {
        static_assert!(0 < 1, 1 < 2, 2 < 3);
    }

    #[test]
    fn test_failure_in_multiple_asserts() {
        // static_assert!(0 < 1, 9999 < 2, 2 < 3); // should not compile
    }

    #[test]
    fn test_integral_args_are_not_supported() {
        // static_assert!(1); // should not compile
    }
}
