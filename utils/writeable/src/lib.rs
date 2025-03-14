// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(test), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic
    )
)]

//! `writeable` is a utility crate of the [`ICU4X`] project.
//!
//! It includes [`Writeable`], a core trait representing an object that can be written to a
//! sink implementing `std::fmt::Write`. It is an alternative to `std::fmt::Display` with the
//! addition of a function indicating the number of bytes to be written.
//!
//! `Writeable` improves upon `std::fmt::Display` in two ways:
//!
//! 1. More efficient, since the sink can pre-allocate bytes.
//! 2. Smaller code, since the format machinery can be short-circuited.
//!
//! Types implementing Writeable have a defaulted [`write_to_string`](Writeable::write_to_string)
//! function. If desired, types implementing `Writeable` can manually implement `ToString`
//! to wrap `write_to_string`.
//!
//! # Examples
//!
//! ```
//! use std::fmt;
//! use writeable::assert_writeable_eq;
//! use writeable::LengthHint;
//! use writeable::Writeable;
//!
//! struct WelcomeMessage<'s> {
//!     pub name: &'s str,
//! }
//!
//! impl<'s> Writeable for WelcomeMessage<'s> {
//!     fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
//!         sink.write_str("Hello, ")?;
//!         sink.write_str(self.name)?;
//!         sink.write_char('!')?;
//!         Ok(())
//!     }
//!
//!     fn write_len(&self) -> LengthHint {
//!         // "Hello, " + '!' + length of name
//!         LengthHint::exact(8 + self.name.len())
//!     }
//! }
//!
//! let message = WelcomeMessage { name: "Alice" };
//! assert_writeable_eq!(&message, "Hello, Alice!");
//! ```
//!
//! [`ICU4X`]: ../icu/index.html

extern crate alloc;

mod impls;
mod ops;

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

/// A hint to help consumers of `Writeable` pre-allocate bytes before they call
/// [`write_to`](Writeable::write_to).
///
/// This behaves like `Iterator::size_hint`: it is a tuple where the first element is the
/// lower bound, and the second element is the upper bound. If the upper bound is `None`
/// either there is no known upper bound, or the upper bound is larger than `usize`.
///
/// `LengthHint` implements std`::ops::{Add, Mul}` and similar traits for easy composition.
/// During computation, the lower bound will saturate at `usize::MAX`, while the upper
/// bound will become `None` if `usize::MAX` is exceeded.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct LengthHint(pub usize, pub Option<usize>);

impl LengthHint {
    pub fn undefined() -> Self {
        Self(0, None)
    }

    /// This is the exact length from `write_to`.
    pub fn exact(n: usize) -> Self {
        Self(n, Some(n))
    }

    /// This is at least the length from `write_to`.
    pub fn at_least(n: usize) -> Self {
        Self(n, None)
    }

    /// This is at most the length from `write_to`.
    pub fn at_most(n: usize) -> Self {
        Self(0, Some(n))
    }

    /// The length from `write_to` is in between `n` and `m`.
    pub fn between(n: usize, m: usize) -> Self {
        Self(Ord::min(n, m), Some(Ord::max(n, m)))
    }

    /// Returns a recommendation for the number of bytes to pre-allocate.
    /// If an upper bound exists, this is used, otherwise the lower bound
    /// (which might be 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use writeable::Writeable;
    ///
    /// fn pre_allocate_string(w: &impl Writeable) -> String {
    ///     String::with_capacity(w.write_len().capacity())
    /// }
    /// ```
    pub fn capacity(&self) -> usize {
        match self {
            Self(lower_bound, None) => *lower_bound,
            Self(_lower_bound, Some(upper_bound)) => *upper_bound,
        }
    }

    /// Returns whether the `LengthHint` indicates that the string is exactly 0 bytes long.
    pub fn is_zero(&self) -> bool {
        self.1 == Some(0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Part {
    pub category: &'static str,
    pub value: &'static str,
}

/// A sink that supports annotating parts of the string with `Part`s.
pub trait PartsWrite: fmt::Write {
    type SubPartsWrite: PartsWrite + ?Sized;

    fn with_part(
        &mut self,
        part: Part,
        f: impl FnMut(&mut Self::SubPartsWrite) -> fmt::Result,
    ) -> fmt::Result;
}

/// `Writeable` is an alternative to `std::fmt::Display` with the addition of a length function.
pub trait Writeable {
    /// Writes bytes to the given sink. Errors from the sink are bubbled up.
    /// The default implementation delegates to `write_to_parts`, and discards any
    /// `Part` annotations.
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        struct CoreWriteAsPartsWrite<W: fmt::Write + ?Sized>(W);
        impl<W: fmt::Write + ?Sized> fmt::Write for CoreWriteAsPartsWrite<W> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.0.write_str(s)
            }

            fn write_char(&mut self, c: char) -> fmt::Result {
                self.0.write_char(c)
            }
        }

        impl<W: fmt::Write + ?Sized> PartsWrite for CoreWriteAsPartsWrite<W> {
            type SubPartsWrite = CoreWriteAsPartsWrite<W>;

            fn with_part(
                &mut self,
                _part: Part,
                mut f: impl FnMut(&mut Self::SubPartsWrite) -> fmt::Result,
            ) -> fmt::Result {
                f(self)
            }
        }

        self.write_to_parts(&mut CoreWriteAsPartsWrite(sink))
    }

    /// Write bytes and `Part` annotations to the given sink. Errors from the
    /// sink are bubbled up. The default implementation delegates to `write_to`,
    /// and doesn't produce any `Part` annotations.
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> fmt::Result {
        self.write_to(sink)
    }

    /// Returns a hint for the number of bytes that will be written to the sink.
    ///
    /// Override this method if it can be computed quickly.
    fn write_len(&self) -> LengthHint {
        LengthHint::undefined()
    }

    /// Creates a new `String` with the data from this `Writeable`. Like `ToString`,
    /// but smaller and faster.
    ///
    /// The default impl allocates an owned `String`. However, if it is possible to return a
    /// borrowed string, overwrite this method to return a `Cow::Borrowed`.
    ///
    /// To remove the `Cow` wrapper, call `.into_owned()`.
    ///
    /// # Examples
    ///
    /// Inspect a `Writeable` before writing it to the sink:
    ///
    /// ```
    /// use core::fmt::{Result, Write};
    /// use writeable::Writeable;
    ///
    /// fn write_if_ascii<W, S>(w: &W, sink: &mut S) -> Result
    /// where
    ///     W: Writeable + ?Sized,
    ///     S: Write + ?Sized,
    /// {
    ///     let s = w.write_to_string();
    ///     if s.is_ascii() {
    ///         sink.write_str(&s)
    ///     } else {
    ///         Ok(())
    ///     }
    /// }
    /// ```
    ///
    /// Convert the `Writeable` into a fully owned `String`:
    ///
    /// ```
    /// use writeable::Writeable;
    ///
    /// fn make_string(w: &impl Writeable) -> String {
    ///     w.write_to_string().into_owned()
    /// }
    /// ```
    fn write_to_string(&self) -> Cow<str> {
        let mut output = String::with_capacity(self.write_len().capacity());
        #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        self.write_to(&mut output)
            .expect("impl Write for String is infallible");
        Cow::Owned(output)
    }
}

/// Testing macros for types implementing Writeable. The first argument should be a
/// `Writeable`, the second argument a string, and the third argument (*_parts_eq only)
/// a list of parts (`[(usize, usize, Part)]`).
///
/// The macros tests for equality of string content, parts (*_parts_eq only), and
/// verify the size hint.
///
/// # Examples
///
/// ```
/// # use writeable::Writeable;
/// # use writeable::LengthHint;
/// # use writeable::Part;
/// # use writeable::assert_writeable_eq;
/// # use writeable::assert_writeable_parts_eq;
/// # use std::fmt::{self, Write};
///
/// const WORD: Part = Part {
///     category: "foo",
///     value: "word",
/// };
///
/// struct Demo;
/// impl Writeable for Demo {
///     fn write_to_parts<S: writeable::PartsWrite + ?Sized>(&self, sink: &mut S) -> fmt::Result {
///         sink.with_part(WORD, |w| w.write_str("foo"))
///     }
///     fn write_len(&self) -> LengthHint {
///         LengthHint::exact(3)
///     }
/// }
///
/// assert_writeable_eq!(&Demo, "foo");
/// assert_writeable_eq!(&Demo, "foo", "Message: {}", "Hello World");
///
/// assert_writeable_parts_eq!(&Demo, "foo", [(0, 3, WORD)]);
/// assert_writeable_parts_eq!(&Demo, "foo", [(0, 3, WORD)], "Message: {}", "Hello World");
/// ```
#[macro_export]
macro_rules! assert_writeable_eq {
    ($actual_writeable:expr, $expected_str:expr $(,)?) => {
        $crate::assert_writeable_eq!($actual_writeable, $expected_str, "");
    };
    ($actual_writeable:expr, $expected_str:expr, $($arg:tt)+) => {{
        let actual_writeable = &$actual_writeable;
        let (actual_str, _) = $crate::writeable_to_parts_for_test(actual_writeable).unwrap();
        assert_eq!(actual_str, $expected_str, $($arg)*);
        assert_eq!(actual_str, $crate::Writeable::write_to_string(actual_writeable), $($arg)+);
        let length_hint = $crate::Writeable::write_len(actual_writeable);
        assert!(length_hint.0 <= actual_str.len(), $($arg)*);
        if let Some(upper) = length_hint.1 {
            assert!(actual_str.len() <= upper, $($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! assert_writeable_parts_eq {
    ($actual_writeable:expr, $expected_str:expr, $expected_parts:expr $(,)?) => {
        $crate::assert_writeable_parts_eq!($actual_writeable, $expected_str, $expected_parts, "");
    };
    ($actual_writeable:expr, $expected_str:expr, $expected_parts:expr, $($arg:tt)+) => {{
        let actual_writeable = &$actual_writeable;
        let (actual_str, actual_parts) = $crate::writeable_to_parts_for_test(actual_writeable).unwrap();
        assert_eq!(actual_str, $expected_str, $($arg)+);
        assert_eq!(actual_str, $crate::Writeable::write_to_string(actual_writeable), $($arg)+);
        assert_eq!(actual_parts, $expected_parts, $($arg)+);
        let length_hint = $crate::Writeable::write_len(actual_writeable);
        assert!(length_hint.0 <= actual_str.len(), $($arg)+);
        if let Some(upper) = length_hint.1 {
            assert!(actual_str.len() <= upper, $($arg)+);
        }
    }};
}

#[doc(hidden)]
#[allow(clippy::type_complexity)]
pub fn writeable_to_parts_for_test<W: Writeable>(
    writeable: &W,
) -> Result<(String, Vec<(usize, usize, Part)>), fmt::Error> {
    struct State {
        string: alloc::string::String,
        parts: Vec<(usize, usize, Part)>,
    }

    impl fmt::Write for State {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.string.write_str(s)
        }
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.string.write_char(c)
        }
    }

    impl PartsWrite for State {
        type SubPartsWrite = Self;
        fn with_part(
            &mut self,
            part: Part,
            mut f: impl FnMut(&mut Self::SubPartsWrite) -> fmt::Result,
        ) -> fmt::Result {
            let start = self.string.len();
            f(self)?;
            self.parts.push((start, self.string.len(), part));
            Ok(())
        }
    }

    let mut state = State {
        string: alloc::string::String::new(),
        parts: Vec::new(),
    };
    writeable.write_to_parts(&mut state)?;

    // Sort by first open and last closed
    state
        .parts
        .sort_unstable_by_key(|(begin, end, _)| (*begin, end.wrapping_neg()));
    Ok((state.string, state.parts))
}
