//! This crate provides a `reduce` function for [`Option<_>`] that allows to
//! merge two options together.
//!
//! This method was previously proposed for addition to `std` two times but both
//! PRs were closed:
//! 1. [#84695][first PR]
//! 2. [#87036][second PR]
//!
//! [first PR]: https://github.com/rust-lang/rust/pull/84695
//! [second PR]: https://github.com/rust-lang/rust/pull/87036
#![no_std]

/// Merges 2 options `a` and `b` together.
///
/// Returns
/// - `Some(f(l, r))` if both options are `Some(_)`
/// - `Some(x)` if either of the options is `Some(x)` and the other is `None`
/// - `None` if both options are `None`
///
/// This is a standalone version of [`OptionExt::reduce`].
///
/// # Examples
///
/// ```
/// use std::cmp::min;
/// use std::ops::Add;
///
/// let x = Some(2);
/// let y = Some(4);
///
/// assert_eq!(opt_reduce::reduce(x, y, Add::add), Some(6));
/// assert_eq!(opt_reduce::reduce(x, y, min), Some(2));
///
/// assert_eq!(opt_reduce::reduce(x, None, Add::add), x);
/// assert_eq!(opt_reduce::reduce(None, y, min), y);
///
/// assert_eq!(opt_reduce::reduce(None, None, i32::add), None);
/// ```
pub fn reduce<T, F>(a: Option<T>, b: Option<T>, f: F) -> Option<T>
where
    F: FnOnce(T, T) -> T,
{
    match (a, b) {
        (Some(l), Some(r)) => Some(f(l, r)),
        (x @ Some(_), None) | (None, x @ Some(_)) => x,
        (None, None) => None,
    }
}

/// [`Option<_>`] extension that adds `reduce` method to it.
pub trait OptionExt {
    /// The generic parameter. I.e. `A` in `Option<A>`.
    type T;

    /// Merges `self` with another `Option`.
    ///
    /// Returns
    /// - `Some(f(l, r))` if both options are `Some(_)`
    /// - `Some(x)` if either of the options is `Some(x)` and the other is
    ///   `None`
    /// - `None` if both options are `None`
    ///
    /// This is an extension version of [`reduce`].
    ///
    /// [`reduce`]: crate::reduce
    ///
    /// # Examples
    ///
    /// ```
    /// use opt_reduce::OptionExt as _;
    /// use std::cmp::min;
    /// use std::ops::Add;
    ///
    /// let x = Some(2);
    /// let y = Some(4);
    ///
    /// assert_eq!(x.reduce(y, Add::add), Some(6));
    /// assert_eq!(x.reduce(y, min), Some(2));
    ///
    /// assert_eq!(x.reduce(None, Add::add), x);
    /// assert_eq!(None.reduce(y, min), y);
    ///
    /// assert_eq!(None.reduce(None, i32::add), None);
    /// ```
    fn reduce<F>(self, other: Option<Self::T>, f: F) -> Option<Self::T>
    where
        F: FnOnce(Self::T, Self::T) -> Self::T;
}

impl<T> OptionExt for Option<T> {
    type T = T;

    fn reduce<F>(self, other: Option<T>, f: F) -> Option<T>
    where
        F: FnOnce(T, T) -> T,
    {
        reduce(self, other, f)
    }
}
