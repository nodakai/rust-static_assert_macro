//! A library implementation of so-called `static_assert` based on a macro which
//! conditionally expands into an integer overflow, breaking the compilation.
//!
//! It can take one or more constant boolean expressions as its parameters.
//!
//! Prior to version 1.1, when you have multiple conditions to verify,
//! you had to rely on the hack to pass them to a single invocation of the macro as multiple parameters
//! because its implementation details prevented multiple invocations from coexisting in a single scope.
//!
//! **This limitation was removed in version 1.1**;
//! you can now call this macro an arbitrary number of times in a given scope.
//!
//! ## Example
//! ```
//! // Rust 2015 syntax
//! #[macro_use]
//! extern crate static_assert_macro;
//!
//! #[cfg(feature = "Rust 2018 syntax")]
//! use static_assert_macro::static_assert;
//!
//! static_assert!(0 < 1);
//! static_assert!(1 < 2);
//! static_assert!(2 < 3, true || false);
//!
//! fn main() {
//!     const FOUR: i32 = 4;
//!     static_assert!(1 + 2 == { FOUR - 1 });
//! }
//! ```

#[macro_export]
macro_rules! static_assert {
    (let $e:expr; ) => (
        const _: [(); { const ASSERT: bool = $e; ASSERT } as usize - 1] = [];
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
