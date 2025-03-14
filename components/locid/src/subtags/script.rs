// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::parser::errors::ParserError;
use core::str::FromStr;
use tinystr::TinyAsciiStr;

/// A script subtag (examples: `"Latn"`, `"Arab"`, etc.)
///
/// [`Script`] represents a Unicode base language code conformat to the
/// [`unicode_script_id`] field of the Language and Locale Identifier.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::Script;
///
/// let script: Script = "Latn".parse().expect("Failed to parse a script subtag.");
/// ```
///
/// [`unicode_script_id`]: https://unicode.org/reports/tr35/#unicode_script_id
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(transparent)]
pub struct Script(TinyAsciiStr<SCRIPT_LENGTH>);

pub const SCRIPT_LENGTH: usize = 4;

impl Script {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed [`Script`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Script;
    ///
    /// let script = Script::from_bytes(b"Latn").expect("Parsing failed.");
    ///
    /// assert_eq!(script, "Latn");
    /// ```
    pub const fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        Self::from_bytes_manual_slice(v, 0, v.len())
    }

    /// Equivalent to [`from_bytes(bytes[start..end])`](Self::from_bytes),
    /// but callable in a `const` context (which range indexing is not).
    pub const fn from_bytes_manual_slice(
        v: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Self, ParserError> {
        if end - start != SCRIPT_LENGTH {
            return Err(ParserError::InvalidSubtag);
        }

        match TinyAsciiStr::from_bytes_manual_slice(v, start, end) {
            Ok(s) if s.is_ascii_alphabetic() => Ok(Self(s.to_ascii_titlecase())),
            _ => Err(ParserError::InvalidSubtag),
        }
    }

    /// Safely creates a [`Script`] from a reference to its raw format
    /// as returned by [`Script::into_raw()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Script;
    ///
    /// assert!(matches!(Script::try_from_raw(*b"Latn"), Ok(_)));
    /// assert!(matches!(Script::try_from_raw(*b"Mymr"), Ok(_)));
    /// assert!(matches!(Script::try_from_raw(*b"1234"), Err(_)));
    ///
    /// // Unlike the other constructors, this one is case-sensitive:
    /// assert!(matches!(Script::try_from_raw(*b"LATN"), Err(_)));
    /// ```
    pub fn try_from_raw(v: [u8; 4]) -> Result<Self, ParserError> {
        let s = TinyAsciiStr::<{ core::mem::size_of::<Self>() }>::try_from_raw(v)
            .map_err(|_| ParserError::InvalidSubtag)?;
        let is_valid = match s.len() {
            SCRIPT_LENGTH => s.is_ascii_alphabetic_titlecase(),
            _ => false,
        };
        if is_valid {
            Ok(Self(s))
        } else {
            Err(ParserError::InvalidSubtag)
        }
    }

    /// Deconstructs the [`Script`] into raw format to be consumed
    /// by [`from_raw_unchecked()`](Script::from_raw_unchecked()).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Script;
    ///
    /// let script = Script::from_bytes(b"Latn").expect("Parsing failed.");
    ///
    /// let raw = script.into_raw();
    /// let script = unsafe { Script::from_raw_unchecked(raw) };
    /// assert_eq!(script, "Latn");
    /// ```
    pub fn into_raw(self) -> [u8; 4] {
        *self.0.all_bytes()
    }

    /// Constructor which takes a raw value returned by
    /// [`into_raw`](Script::into_raw()).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Script;
    ///
    /// let script = Script::from_bytes(b"Latn").expect("Parsing failed.");
    ///
    /// let raw = script.into_raw();
    /// let script = unsafe { Script::from_raw_unchecked(raw) };
    /// assert_eq!(script, "Latn");
    /// ```
    ///
    /// # Safety
    ///
    /// This function accepts a [u8; 4] that is expected to be a valid [`TinyAsciiStr<4>`]
    /// representing a [`Script`] subtag in canonical syntax.
    pub const unsafe fn from_raw_unchecked(v: [u8; 4]) -> Self {
        Self(TinyAsciiStr::from_bytes_unchecked(v))
    }

    /// A helper function for displaying
    /// a [`Script`] subtag as a `&`[`str`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Script;
    ///
    /// let script = Script::from_bytes(b"Latn").expect("Parsing failed.");
    ///
    /// assert_eq!(script.as_str(), "Latn");
    /// ```
    ///
    /// `Notice`: For many use cases, such as comparison,
    /// [`Script`] implements [`PartialEq`]`<&`[`str`]`>` which allows for direct comparisons.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for Script {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl_writeable_for_single_subtag!(Script, "Mymr");

impl PartialEq<&str> for Script {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<'l> From<&'l Script> for &'l str {
    fn from(input: &'l Script) -> Self {
        input.as_str()
    }
}

impl From<Script> for TinyAsciiStr<4> {
    fn from(input: Script) -> Self {
        input.0
    }
}
