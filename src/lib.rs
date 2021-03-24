//! This crate entroduces [`is_false!`] macro and [`is_false()`] function which checks if argument is true

/// Macro checks if all provided expressions are false
/// ## Examples
/// ```rust
/// use is_false::is_false;
///
/// fn main() {
///    assert_eq!(is_false!(4 % 2 == 0, 5 % 2 == 0), false);        
///    assert_eq!(is_false!(5 % 2 == 0, 1 == 2), true);
/// }
/// ```
/// ```rust
/// use is_false::is_false;
///
/// fn main() {
///     assert_eq!(is_false!(false), true);
/// }
/// ```

#[macro_export]
macro_rules! is_false {
    ( $( $x:expr ),* ) => {
        {
            is_true::is_true!($(!$x),*)
        }
    };
}

/// Function checks if argument is true
pub fn is_false(arg: bool) -> bool {
    !is_true::is_true(arg)
}

#[cfg(test)]
mod tests {
    use crate::is_false;

    #[test]
    fn yes_its_true() {
        assert_eq!(is_false(false), true);
        assert_eq!(is_false!(false), true);
    }
    #[test]
    fn no_its_false() {
        assert_eq!(is_false(true), false);
        assert_eq!(is_false!(true), false);
    }
    #[test]
    fn multiple() {
        assert_eq!(is_false!(false, false, true), false);
        assert_eq!(is_false!(false, false, false), true);
    }
}