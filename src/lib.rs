//! Small library providing some macros helpful for asserting.

/// Panics if the two expressions are equal. Requires that the types be
/// comparable with `!=`.
///
/// Prints the values out on panic.
#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val != *right_val) {
                    panic!("assertion failed: `(left != right)` (left: `{:?}`, right: `{:?}`)", left_val, right_val)
                }
            }
        }
    })
}

/// Panics if the first expression is not strictly less than the second.
/// Requires that the types be comparable with `<`.
///
/// Prints the values out on panic.
#[macro_export]
macro_rules! assert_lt {
    ($left:expr, $right:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val < *right_val) {
                    panic!("assertion failed: `(left < right)` (left: `{:?}`, right: `{:?}`)", left_val, right_val)
                }
            }
        }
    })
}

/// Panics if the first expression is not strictly greater than the second.
/// Requires that the types be comparable with `>`.
///
/// Prints the values out on panic.
#[macro_export]
macro_rules! assert_gt {
    ($left:expr, $right:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val > *right_val) {
                    panic!("assertion failed: `(left > right)` (left: `{:?}`, right: `{:?}`)", left_val, right_val)
                }
            }
        }
    })
}

/// Panics if the first expression is not less than or equal to the second.
/// Requires that the types be comparable with `<=`.
///
/// Prints the values out on panic.
#[macro_export]
macro_rules! assert_le {
    ($left:expr, $right:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val <= *right_val) {
                    panic!("assertion failed: `(left <= right)` (left: `{:?}`, right: `{:?}`)", left_val, right_val)
                }
            }
        }
    })
}

/// Panics if the first expression is not greater than or equal to the second.
/// Requires that the types be comparable with `>=`.
///
/// Prints the values out on panic.
#[macro_export]
macro_rules! assert_ge {
    ($left:expr, $right:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val >= *right_val) {
                    panic!("assertion failed: `(left >= right)` (left: `{:?}`, right: `{:?}`)", left_val, right_val)
                }
            }
        }
    })
}

/// Panics if the two expressions are equal. Requires that the types be
/// comparable with `!=`.
///
/// Prints the values out on panic.
///
/// This macro is disabled by default in optimized builds, unless 
/// `-C debug-assertions` is provided to the compiler.
#[macro_export]
macro_rules! debug_assert_ne {
    ($left:expr, $right:expr) => {
        if cfg!(debug_assertions) {
            assert_ne!($left, $right);
        }
    }
}

/// Panics if the first expression is not strictly less than the second.
/// Requires that the types be comparable with `<`.
///
/// Prints the values out on panic.
///
/// This macro is disabled by default in optimized builds, unless 
/// `-C debug-assertions` is provided to the compiler.
#[macro_export]
macro_rules! debug_assert_lt {
    ($left:expr, $right:expr) => {
        if cfg!(debug_assertions) {
            assert_lt!($left, $right);
        }
    }
}

/// Panics if the first expression is not strictly greater than the second.
/// Requires that the types be comparable with `>`.
///
/// Prints the values out on panic.
///
/// This macro is disabled by default in optimized builds, unless 
/// `-C debug-assertions` is provided to the compiler.
#[macro_export]
macro_rules! debug_assert_gt {
    ($left:expr, $right:expr) => {
        if cfg!(debug_assertions) {
            assert_gt!($left, $right);
        }
    }
}

/// Panics if the first expression is not less than or equal to the second.
/// Requires that the types be comparable with `<=`.
///
/// Prints the values out on panic.
///
/// This macro is disabled by default in optimized builds, unless 
/// `-C debug-assertions` is provided to the compiler.
#[macro_export]
macro_rules! debug_assert_le {
    ($left:expr, $right:expr) => {
        if cfg!(debug_assertions) {
            assert_le!($left, $right);
        }
    }
}

/// Panics if the first expression is not greater than or equal to the second.
/// Requires that the types be comparable with `>=`.
///
/// Prints the values out on panic.
///
/// This macro is disabled by default in optimized builds, unless 
/// `-C debug-assertions` is provided to the compiler.
#[macro_export]
macro_rules! debug_assert_ge {
    ($left:expr, $right:expr) => {
        if cfg!(debug_assertions) {
            assert_ge!($left, $right);
        }
    }
}


/// Panics if reached. This is a variant of the standard library's `unreachable!`
/// macro that is controlled by `cfg!(debug_assertations)`.
///
/// Useful mainly when reaching the code is a bug that you absolutely want to
/// know about, but there's a sane way to proceed in production code regardless.
/// Forwards any arguments passed to it to `unreachable!`.
#[macro_export]
macro_rules! debug_unreachable {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            unreachable!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_ne_pass() {
        let a = 3;
        assert_ne!(a, 4);
        assert_ne!("foo", "bar");
    }

    #[test]
    #[should_panic]
    fn test_assert_ne_fail() {
        assert_ne!(3, 3);
    }

    #[test]
    fn test_assert_lt_pass() {
        let a = 3;
        assert_lt!(a, 4);
        assert_lt!(4.0, 4.5);
    }

    #[test]
    #[should_panic]
    fn test_assert_lt_fail0() {
        assert_lt!(3, 3);
    }

    #[test]
    #[should_panic]
    fn test_assert_lt_fail1() {
        assert_lt!(5, 3);
    }


    #[test]
    fn test_assert_le_pass() {
        let a = 3;
        assert_le!(a, 4);
        assert_le!(4.0, 4.5);
        assert_le!(10, 10);
    }

    #[test]
    #[should_panic]
    fn test_assert_le_fail() {
        assert_le!(5, 3);
    }

    #[test]
    fn test_assert_gt_pass() {
        let a = 3;
        assert_gt!(4, a);
        assert_gt!(4.5, 4.0);
    }

    #[test]
    #[should_panic]
    fn test_assert_gt_fail0() {
        assert_gt!(3, 3);
    }

    #[test]
    #[should_panic]
    fn test_assert_gt_fail1() {
        assert_gt!(3, 5);
    }


    #[test]
    fn test_assert_ge_pass() {
        let a = 3;
        assert_ge!(4, a);
        assert_ge!(4.5, 4.0);
        assert_ge!(10, 10);
    }

    #[test]
    #[should_panic]
    fn test_assert_ge_fail() {
        assert_ge!(3, 5);
    }
    // @@TODO: how to test the debug macros?
}
