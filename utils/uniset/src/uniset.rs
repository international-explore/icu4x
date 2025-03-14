// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "serde")]
use alloc::format;
use alloc::vec::Vec;
use core::{char, ops::RangeBounds, ops::RangeInclusive};
use yoke::Yokeable;
use zerofrom::ZeroFrom;
use zerovec::{ule::AsULE, ZeroSlice, ZeroVec};

use super::UnicodeSetError;
use crate::utils::{deconstruct_range, is_valid_zv};

/// Represents the end code point of the Basic Multilingual Plane range, starting from code point 0, inclusive
const BMP_MAX: u32 = 0xFFFF;

/// Represents the inversion list for a set of all code points in the Basic Multilingual Plane.
const BMP_INV_LIST_SLICE: &ZeroSlice<u32> =
    ZeroSlice::<u32>::from_ule_slice_const(&<u32 as AsULE>::ULE::from_array([0x0, BMP_MAX + 1]));

/// Represents the inversion list for all of the code points in the Unicode range.
const ALL_SLICE: &ZeroSlice<u32> =
    ZeroSlice::<u32>::from_ule_slice_const(&<u32 as AsULE>::ULE::from_array([
        0x0,
        (char::MAX as u32) + 1,
    ]));

/// A membership wrapper for [`UnicodeSet`].
///
/// Provides exposure to membership functions and constructors from serialized [`UnicodeSets`](UnicodeSet)
/// and predefined ranges.
#[derive(Debug, Eq, PartialEq, Clone, Yokeable, ZeroFrom)]
pub struct UnicodeSet<'data> {
    // If we wanted to use an array to keep the memory on the stack, there is an unsafe nightly feature
    // https://doc.rust-lang.org/nightly/core/array/trait.FixedSizeArray.html
    // Allows for traits of fixed size arrays

    // Implements an [inversion list.](https://en.wikipedia.org/wiki/Inversion_list)
    inv_list: ZeroVec<'data, u32>,
    size: usize,
}

#[cfg(any(feature = "serde", test))]
impl<'de: 'a, 'a> serde::Deserialize<'de> for UnicodeSet<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let parsed_inv_list = ZeroVec::<u32>::deserialize(deserializer)?;

        UnicodeSet::from_inversion_list(parsed_inv_list).map_err(|e| {
            Error::custom(format!(
                "Cannot deserialize invalid inversion list for UnicodeSet: {:?}",
                e
            ))
        })
    }
}

#[cfg(feature = "crabbake")]
impl crabbake::Bakeable for UnicodeSet<'_> {
    fn bake(&self, env: &crabbake::CrateEnv) -> crabbake::TokenStream {
        env.insert("icu_uniset");
        let inv_list = self.inv_list.bake(env);
        let size = self.size.bake(env);
        // Safe because our parts are safe.
        crabbake::quote! { unsafe {
            #[allow(unused_unsafe)]
            ::icu_uniset::UnicodeSet::from_parts_unchecked(#inv_list, #size)
        }}
    }
}

// Note: serde(flatten) currently does not promote a struct field of type Vec
// to replace the struct when serializing. The error message from the default
// serialization is: "can only flatten structs and maps (got a sequence)".

#[cfg(any(feature = "serde_serialize", test))]
impl<'data> serde::Serialize for UnicodeSet<'data> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inv_list.serialize(serializer)
    }
}

impl<'data> UnicodeSet<'data> {
    /// Returns a new [`UnicodeSet`] from an [inversion list](https://en.wikipedia.org/wiki/Inversion_list)
    /// represented as a [`ZeroVec`]`<`[`u32`]`>` of code points.
    ///
    /// The inversion list must be of even length, sorted ascending non-overlapping,
    /// and within the bounds of `0x0 -> 0x10FFFF` inclusive, and end points being exclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// use icu_uniset::UnicodeSetError;
    /// use zerovec::ZeroVec;
    /// let valid = [0x0, 0x10000];
    /// let inv_list: ZeroVec<u32> = ZeroVec::from_slice_or_alloc(&valid);
    /// let result = UnicodeSet::from_inversion_list(inv_list);
    /// assert!(matches!(result, UnicodeSet));
    ///
    /// let invalid: Vec<u32> = vec![0x0, 0x80, 0x3];
    /// let inv_list: ZeroVec<u32> = ZeroVec::from_slice_or_alloc(&invalid);
    /// let result = UnicodeSet::from_inversion_list(inv_list);
    /// assert!(matches!(result, Err(UnicodeSetError::InvalidSet(_))));
    /// if let Err(UnicodeSetError::InvalidSet(actual)) = result {
    ///     assert_eq!(&invalid, &actual);
    /// }
    /// ```
    pub fn from_inversion_list(inv_list: ZeroVec<'data, u32>) -> Result<Self, UnicodeSetError> {
        #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
        if is_valid_zv(&inv_list) {
            let size: usize = inv_list
                .as_ule_slice()
                .chunks(2)
                .map(|end_points| {
                    <u32 as AsULE>::from_unaligned(end_points[1])
                        - <u32 as AsULE>::from_unaligned(end_points[0])
                })
                .sum::<u32>() as usize;
            Ok(Self { inv_list, size })
        } else {
            Err(UnicodeSetError::InvalidSet(inv_list.to_vec()))
        }
    }

    #[doc(hidden)] // Crabbake internal
    pub const unsafe fn from_parts_unchecked(inv_list: ZeroVec<'data, u32>, size: usize) -> Self {
        Self { inv_list, size }
    }

    /// Returns a new [`UnicodeSet`] by borrowing an [inversion list](https://en.wikipedia.org/wiki/Inversion_list)
    /// represented as a slice of [`u32`] code points.
    ///
    /// The inversion list must be of even length, sorted ascending non-overlapping,
    /// and within the bounds of `0x0 -> 0x10FFFF` inclusive, and end points being exclusive.
    ///
    /// Note: The slice may be cloned on certain platforms; for more information, see [`ZeroVec::from_slice_or_alloc`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// use icu_uniset::UnicodeSetError;
    /// use zerovec::ZeroVec;
    /// let valid = [0x0, 0x10000];
    /// let result = UnicodeSet::from_inversion_list_slice(&valid);
    /// assert!(matches!(result, UnicodeSet));
    ///
    /// let invalid: Vec<u32> = vec![0x0, 0x80, 0x3];
    /// let result = UnicodeSet::from_inversion_list_slice(&invalid);
    /// assert!(matches!(result, Err(UnicodeSetError::InvalidSet(_))));
    /// if let Err(UnicodeSetError::InvalidSet(actual)) = result {
    ///     assert_eq!(&invalid, &actual);
    /// }
    /// ```
    pub fn from_inversion_list_slice(inv_list: &'data [u32]) -> Result<Self, UnicodeSetError> {
        let inv_list_zv: ZeroVec<u32> = ZeroVec::from_slice_or_alloc(inv_list);
        UnicodeSet::from_inversion_list(inv_list_zv)
    }

    /// Returns a new, fully-owned [`UnicodeSet`] by cloning an [inversion list](https://en.wikipedia.org/wiki/Inversion_list)
    /// represented as a slice of [`u32`] code points.
    ///
    /// The inversion list must be of even length, sorted ascending non-overlapping,
    /// and within the bounds of `0x0 -> 0x10FFFF` inclusive, and end points being exclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// use icu_uniset::UnicodeSetError;
    /// use zerovec::ZeroVec;
    ///
    /// use std::vec::Vec;
    ///
    /// fn inv_list_to_owned_unicodeset(inv_list: &[u32]) -> UnicodeSet {
    ///     UnicodeSet::clone_from_inversion_list_slice(inv_list).unwrap()
    /// }
    ///
    /// let bmp_list: [u32; 2] = [0x0, 0x10000];
    /// let smp_list: Vec<u32> = vec![0x10000, 0x20000];
    /// let sip_list: &[u32] = &vec![0x20000, 0x30000];
    ///
    /// let inv_lists: [&[u32]; 3] = [&bmp_list, &smp_list, sip_list];
    /// let unicodesets: Vec<UnicodeSet> = inv_lists
    ///     .iter()
    ///     .map(|il| inv_list_to_owned_unicodeset(il))
    ///     .collect();
    ///
    /// let bmp = &unicodesets.get(0).unwrap();
    /// assert!(bmp.contains_u32(0xFFFF));
    /// assert!(!bmp.contains_u32(0x10000));
    ///
    /// assert!(!&unicodesets.iter().any(|set| set.contains_u32(0x40000)));
    /// ```
    pub fn clone_from_inversion_list_slice(inv_list: &[u32]) -> Result<Self, UnicodeSetError> {
        let inv_list_zv: ZeroVec<u32> = ZeroVec::alloc_from_slice(inv_list);
        UnicodeSet::from_inversion_list(inv_list_zv)
    }

    /// Returns an owned inversion list representing the current [`UnicodeSet`]
    pub fn get_inversion_list(&self) -> Vec<u32> {
        let result: Vec<u32> = self.as_inversion_list().to_vec(); // Only crate public, to not leak impl
        result
    }

    /// Returns [`UnicodeSet`] spanning entire Unicode range
    ///
    /// The range spans from `0x0 -> 0x10FFFF` inclusive.
    ///  
    /// # Examples
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// use zerovec::ZeroVec;
    ///
    /// let expected = vec![0x0, (char::MAX as u32) + 1];
    /// assert_eq!(UnicodeSet::all().get_inversion_list(), expected);
    /// assert_eq!(
    ///     UnicodeSet::all().size(),
    ///     (expected[1] - expected[0]) as usize
    /// );
    /// ```
    pub fn all() -> Self {
        Self {
            inv_list: ALL_SLICE.as_zerovec(),
            size: (char::MAX as usize) + 1,
        }
    }

    /// Returns [`UnicodeSet`] spanning BMP range
    ///
    /// The range spans from `0x0 -> 0xFFFF` inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// use zerovec::ZeroVec;
    ///
    /// const BMP_MAX: u32 = 0xFFFF;
    ///
    /// let expected = vec![0x0, BMP_MAX + 1];
    /// assert_eq!(UnicodeSet::bmp().get_inversion_list(), expected);
    /// assert_eq!(
    ///     UnicodeSet::bmp().size(),
    ///     (expected[1] - expected[0]) as usize
    /// );
    /// ```
    pub fn bmp() -> Self {
        Self {
            inv_list: BMP_INV_LIST_SLICE.as_zerovec(),
            size: (BMP_MAX as usize) + 1,
        }
    }

    /// Returns the inversion list as a slice
    ///
    /// Public only to the crate, not exposed to public
    pub(crate) fn as_inversion_list(&self) -> &ZeroVec<u32> {
        &self.inv_list
    }

    /// Yields an [`Iterator`] going through the character set in the [`UnicodeSet`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// let example_list = [0x41, 0x44, 0x45, 0x46];
    /// let example = UnicodeSet::from_inversion_list_slice(&example_list).unwrap();
    /// let mut ex_iter_chars = example.iter_chars();
    /// assert_eq!(Some('A'), ex_iter_chars.next());
    /// assert_eq!(Some('B'), ex_iter_chars.next());
    /// assert_eq!(Some('C'), ex_iter_chars.next());
    /// assert_eq!(Some('E'), ex_iter_chars.next());
    /// assert_eq!(None, ex_iter_chars.next());
    /// ```
    pub fn iter_chars(&self) -> impl Iterator<Item = char> + '_ {
        #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
        self.inv_list
            .as_ule_slice()
            .chunks(2)
            .flat_map(|pair| (AsULE::from_unaligned(pair[0])..AsULE::from_unaligned(pair[1])))
            .filter_map(char::from_u32)
    }

    /// Yields an [`Iterator`] returning the ranges of the code points that are
    /// included in the [`UnicodeSet`]
    ///
    /// Ranges are returned as [`RangeInclusive`], which is inclusive of its
    /// `end` bound value. An end-inclusive behavior matches the ICU4C/J
    /// behavior of ranges, ex: `UnicodeSet::contains(UChar32 start, UChar32 end)`.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// let example_list = [0x41, 0x44, 0x45, 0x46];
    /// let example = UnicodeSet::from_inversion_list_slice(&example_list).unwrap();
    /// let mut example_iter_ranges = example.iter_ranges();
    /// assert_eq!(Some(0x41..=0x43), example_iter_ranges.next());
    /// assert_eq!(Some(0x45..=0x45), example_iter_ranges.next());
    /// assert_eq!(None, example_iter_ranges.next());
    /// ```
    pub fn iter_ranges(&self) -> impl ExactSizeIterator<Item = RangeInclusive<u32>> + '_ {
        #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
        self.inv_list.as_ule_slice().chunks(2).map(|pair| {
            let range_start: u32 = AsULE::from_unaligned(pair[0]);
            let range_limit: u32 = AsULE::from_unaligned(pair[1]);
            RangeInclusive::new(range_start, range_limit - 1)
        })
    }

    /// Returns the number of ranges contained in this [`UnicodeSet`]
    pub fn get_range_count(&self) -> usize {
        self.inv_list.len() / 2
    }

    /// Returns a specific range contained in this [`UnicodeSet`] by index.
    /// Intended for use in FFI.
    pub fn get_nth_range(&self, idx: usize) -> Option<RangeInclusive<u32>> {
        let start_idx = idx * 2;
        let end_idx = start_idx + 1;
        let start = self.inv_list.get(start_idx)?;
        let end = self.inv_list.get(end_idx)?;
        Some(RangeInclusive::new(start, end - 1))
    }

    /// Returns the number of elements of the [`UnicodeSet`]
    pub fn size(&self) -> usize {
        if self.is_empty() {
            return 0;
        }
        self.size
    }

    /// Returns whether or not the [`UnicodeSet`] is empty
    pub fn is_empty(&self) -> bool {
        self.inv_list.is_empty()
    }

    /// Wrapper for contains
    ///
    /// Returns an [`Option`] as to whether or not it is possible for the query to be contained.
    /// The value in the [`Option`] is the start index of the range that contains the query.
    fn contains_query(&self, query: u32) -> Option<usize> {
        match self.inv_list.binary_search(&query) {
            Ok(pos) => {
                if pos % 2 == 0 {
                    Some(pos)
                } else {
                    None
                }
            }
            Err(pos) => {
                if pos % 2 != 0 && pos < self.inv_list.len() {
                    Some(pos - 1)
                } else {
                    None
                }
            }
        }
    }

    /// Checks to see the query is in the [`UnicodeSet`]
    ///
    /// Runs a binary search in `O(log(n))` where `n` is the number of start and end points
    /// in the set using [`core`] implementation
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// let example_list = [0x41, 0x43, 0x44, 0x45];
    /// let example = UnicodeSet::from_inversion_list_slice(&example_list).unwrap();
    /// assert!(example.contains('A'));
    /// assert!(!example.contains('C'));
    /// ```
    pub fn contains(&self, query: char) -> bool {
        self.contains_query(query as u32).is_some()
    }

    /// Checks to see the unsigned int is in the [`UnicodeSet::all()`](UnicodeSet::all())
    ///
    /// Note: Even though [`u32`] and [`prim@char`] in Rust are non-negative 4-byte
    /// values, there is an important difference. A [`u32`] can take values up to
    /// a very large integer value, while a [`prim@char`] in Rust is defined to be in
    /// the range from 0 to the maximum valid Unicode Scalar Value.
    ///
    /// Runs a binary search in `O(log(n))` where `n` is the number of start and end points
    /// in the set using [`core`] implementation
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// let example_list = [0x41, 0x43, 0x44, 0x45];
    /// let example = UnicodeSet::from_inversion_list_slice(&example_list).unwrap();
    /// assert!(example.contains_u32(0x41));
    /// assert!(!example.contains_u32(0x43));
    /// ```
    pub fn contains_u32(&self, query: u32) -> bool {
        self.contains_query(query).is_some()
    }

    /// Checks to see if the range is in the [`UnicodeSet`]
    ///
    /// Runs a binary search in `O(log(n))` where `n` is the number of start and end points
    /// in the set using [`Vec`] implementation. Only runs the search once on the `start`
    /// parameter, while the `end` parameter is checked in a single `O(1)` step.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// let example_list = [0x41, 0x43, 0x44, 0x45];
    /// let example = UnicodeSet::from_inversion_list_slice(&example_list).unwrap();
    /// assert!(example.contains_range(&('A'..'C')));
    /// assert!(example.contains_range(&('A'..='B')));
    /// assert!(!example.contains_range(&('A'..='C')));
    /// ```
    ///
    /// Surrogate points (`0xD800 -> 0xDFFF`) will return [`false`] if the Range contains them but the
    /// [`UnicodeSet`] does not.
    ///
    /// Note: when comparing to ICU4C/J, keep in mind that `Range`s in Rust are
    /// constructed inclusive of start boundary and exclusive of end boundary.
    /// The ICU4C/J `UnicodeSet::contains(UChar32 start, UChar32 end)` method
    /// differs by including the end boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// use std::char;
    /// let check = char::from_u32(0xD7FE).unwrap()..char::from_u32(0xE001).unwrap();
    /// let example_list = [0xD7FE, 0xD7FF, 0xE000, 0xE001];
    /// let example = UnicodeSet::from_inversion_list_slice(&example_list).unwrap();
    /// assert!(!example.contains_range(&(check)));
    /// ```
    pub fn contains_range(&self, range: &impl RangeBounds<char>) -> bool {
        let (from, till) = deconstruct_range(range);
        if from >= till {
            return false;
        }
        match self.contains_query(from) {
            Some(pos) => {
                if let Some(x) = self.inv_list.get(pos + 1) {
                    (till) <= x
                } else {
                    debug_assert!(
                        false,
                        "Inversion list query should not return out of bounds index"
                    );
                    false
                }
            }
            None => false,
        }
    }

    /// Check if the calling [`UnicodeSet`] contains all the characters of the given [`UnicodeSet`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// let example_list = [0x41, 0x46, 0x55, 0x5B]; // A - E, U - Z
    /// let example = UnicodeSet::from_inversion_list_slice(&example_list).unwrap();
    /// let a_to_d = UnicodeSet::from_inversion_list_slice(&[0x41, 0x45]).unwrap();
    /// let f_to_t = UnicodeSet::from_inversion_list_slice(&[0x46, 0x55]).unwrap();
    /// let r_to_x = UnicodeSet::from_inversion_list_slice(&[0x52, 0x58]).unwrap();
    /// assert!(example.contains_set(&a_to_d)); // contains all
    /// assert!(!example.contains_set(&f_to_t)); // contains none
    /// assert!(!example.contains_set(&r_to_x)); // contains some
    /// ```
    pub fn contains_set(&self, set: &Self) -> bool {
        if set.size() > self.size() {
            return false;
        }

        let mut set_ranges = set.iter_ranges();
        let mut check_elem = set_ranges.next();

        let ranges = self.iter_ranges();
        for range in ranges {
            match check_elem {
                Some(ref check_range) => {
                    if check_range.start() >= range.start()
                        && check_range.end() <= &(range.end() + 1)
                    {
                        check_elem = set_ranges.next();
                    }
                }
                _ => break,
            }
        }
        check_elem.is_none()
    }

    /// Returns the end of the initial substring where the characters are either contained/not contained
    /// in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// let example_list = [0x41, 0x44]; // {A, B, C}
    /// let example = UnicodeSet::from_inversion_list_slice(&example_list).unwrap();
    /// assert_eq!(example.span("CABXYZ", true), 3);
    /// assert_eq!(example.span("XYZC", false), 3);
    /// assert_eq!(example.span("XYZ", true), 0);
    /// assert_eq!(example.span("ABC", false), 0);
    /// ```
    pub fn span(&self, span_str: &str, contained: bool) -> usize {
        span_str
            .chars()
            .take_while(|&x| self.contains(x) == contained)
            .count()
    }

    /// Returns the start of the trailing substring (starting from end of string) where the characters are
    /// either contained/not contained in the set. Returns the length of the string if no valid return.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// let example_list = [0x41, 0x44]; // {A, B, C}
    /// let example = UnicodeSet::from_inversion_list_slice(&example_list).unwrap();
    /// assert_eq!(example.span_back("XYZCAB", true), 3);
    /// assert_eq!(example.span_back("ABCXYZ", true), 6);
    /// assert_eq!(example.span_back("CABXYZ", false), 3);
    /// ```
    pub fn span_back(&self, span_str: &str, contained: bool) -> usize {
        span_str.len()
            - span_str
                .chars()
                .rev()
                .take_while(|&x| self.contains(x) == contained)
                .count()
    }
}

#[cfg(test)]
mod tests {
    use super::{UnicodeSet, UnicodeSetError};
    use std::{char, vec::Vec};
    use zerovec::ZeroVec;

    #[test]
    fn test_unicodeset_try_from_vec() {
        let ex = vec![0x2, 0x3, 0x4, 0x5];
        let check = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        assert_eq!(ex, check.get_inversion_list());
        assert_eq!(2, check.size());
    }

    #[test]
    fn test_unicodeset_try_from_vec_error() {
        let check = vec![0x1, 0x1, 0x2, 0x3, 0x4];
        let inv_list = ZeroVec::from_slice_or_alloc(&check);
        let set = UnicodeSet::from_inversion_list(inv_list);
        assert!(matches!(set, Err(UnicodeSetError::InvalidSet(_))));
        if let Err(UnicodeSetError::InvalidSet(actual)) = set {
            assert_eq!(&check, &actual);
        }
    }

    // UnicodeSet membership functions
    #[test]
    fn test_unicodeset_contains_query() {
        let ex = vec![0x41, 0x46, 0x4B, 0x55];
        let check = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        assert!(check.contains_query(0x40).is_none());
        assert_eq!(check.contains_query(0x41).unwrap(), 0);
        assert_eq!(check.contains_query(0x44).unwrap(), 0);
        assert!(check.contains_query(0x46).is_none());
        assert_eq!(check.contains_query(0x4C).unwrap(), 2);
        assert!(check.contains_query(0x56).is_none());
    }

    #[test]
    fn test_unicodeset_contains() {
        let ex = vec![0x2, 0x5, 0xA, 0xF];
        let check = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        assert!(check.contains(0x2 as char));
        assert!(check.contains(0x4 as char));
        assert!(check.contains(0xA as char));
        assert!(check.contains(0xE as char));
    }

    #[test]
    fn test_unicodeset_contains_false() {
        let ex = vec![0x2, 0x5, 0xA, 0xF];
        let check = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        assert!(!check.contains(0x1 as char));
        assert!(!check.contains(0x5 as char));
        assert!(!check.contains(0x9 as char));
        assert!(!check.contains(0xF as char));
        assert!(!check.contains(0x10 as char));
    }

    #[test]
    fn test_unicodeset_contains_range() {
        let ex = vec![0x41, 0x46, 0x4B, 0x55];
        let check = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        assert!(check.contains_range(&('A'..='E'))); // 65 - 69
        assert!(check.contains_range(&('C'..'D'))); // 67 - 67
        assert!(check.contains_range(&('L'..'P'))); // 76 - 80
        assert!(!check.contains_range(&('L'..='U'))); // 76 - 85
    }

    #[test]
    fn test_unicodeset_contains_range_false() {
        let ex = vec![0x41, 0x46, 0x4B, 0x55];
        let check = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        assert!(!check.contains_range(&('!'..'A'))); // 33 - 65
        assert!(!check.contains_range(&('F'..'K'))); // 70 - 74
        assert!(!check.contains_range(&('U'..))); // 85 - ..
    }

    #[test]
    fn test_unicodeset_contains_range_invalid() {
        let check = UnicodeSet::all();
        assert!(!check.contains_range(&('A'..'!'))); // 65 - 33
        assert!(!check.contains_range(&('A'..'A'))); // 65 - 65
    }

    #[test]
    fn test_unicodeset_contains_set_u() {
        let ex = vec![0xA, 0x14, 0x28, 0x32, 0x46, 0x50, 0x64, 0x6E];
        let u = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        let inside = vec![0xF, 0x14, 0x2C, 0x31, 0x46, 0x50, 0x64, 0x6D];
        let s = UnicodeSet::from_inversion_list_slice(&inside).unwrap();
        assert!(u.contains_set(&s));
    }

    #[test]
    fn test_unicodeset_contains_set_u_false() {
        let ex = vec![0xA, 0x14, 0x28, 0x32, 0x46, 0x50, 0x64, 0x78];
        let u = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        let outside = vec![0x0, 0xA, 0x16, 0x2C, 0x32, 0x46, 0x4F, 0x51, 0x6D, 0x6F];
        let s = UnicodeSet::from_inversion_list_slice(&outside).unwrap();
        assert!(!u.contains_set(&s));
    }

    #[test]
    fn test_unicodeset_size() {
        let ex = vec![0x2, 0x5, 0xA, 0xF];
        let check = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        assert_eq!(8, check.size());
        let check = UnicodeSet::all();
        let expected = (char::MAX as u32) + 1;
        assert_eq!(expected as usize, check.size());
        let inv_list_vec: Vec<u32> = vec![];
        let check = UnicodeSet {
            inv_list: ZeroVec::from_slice_or_alloc(&inv_list_vec),
            size: 0,
        };
        assert_eq!(check.size(), 0);
    }

    #[test]
    fn test_unicodeset_is_empty() {
        let inv_list_vec: Vec<u32> = vec![];
        let check = UnicodeSet {
            inv_list: ZeroVec::from_slice_or_alloc(&inv_list_vec),
            size: 0,
        };
        assert!(check.is_empty());
    }

    #[test]
    fn test_unicodeset_is_not_empty() {
        let check = UnicodeSet::all();
        assert!(!check.is_empty());
    }

    #[test]
    fn test_unicodeset_iter_chars() {
        let ex = vec![0x41, 0x44, 0x45, 0x46, 0xD800, 0xD801];
        let check = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        let mut iter = check.iter_chars();
        assert_eq!(Some('A'), iter.next());
        assert_eq!(Some('B'), iter.next());
        assert_eq!(Some('C'), iter.next());
        assert_eq!(Some('E'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_unicodeset_iter_ranges() {
        let ex = vec![0x41, 0x44, 0x45, 0x46, 0xD800, 0xD801];
        let set = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        let mut ranges = set.iter_ranges();
        assert_eq!(Some(0x41..=0x43), ranges.next());
        assert_eq!(Some(0x45..=0x45), ranges.next());
        assert_eq!(Some(0xD800..=0xD800), ranges.next());
        assert_eq!(None, ranges.next());
    }

    #[test]
    fn test_unicodeset_iter_ranges_exactsizeiter_trait() {
        let ex = vec![0x41, 0x44, 0x45, 0x46, 0xD800, 0xD801];
        let set = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        let ranges = set.iter_ranges();
        assert_eq!(3, ranges.len());
    }

    #[test]
    fn test_unicodeset_range_count() {
        let ex = vec![0x41, 0x44, 0x45, 0x46, 0xD800, 0xD801];
        let set = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        assert_eq!(3, set.get_range_count());
    }

    #[test]
    fn test_unicodeset_get_nth_range() {
        let ex = vec![0x41, 0x44, 0x45, 0x46, 0xD800, 0xD801];
        let set = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        assert_eq!(Some(0x41..=0x43), set.get_nth_range(0));
        assert_eq!(Some(0x45..=0x45), set.get_nth_range(1));
        assert_eq!(Some(0xD800..=0xD800), set.get_nth_range(2));
        assert_eq!(None, set.get_nth_range(3));
    }

    // Range<char> cannot represent the upper bound (non-inclusive) for
    // char::MAX, whereas Range<u32> can.
    #[test]
    fn test_unicodeset_iter_ranges_with_max_code_point() {
        let ex = vec![0x80, (char::MAX as u32) + 1];
        let set = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        let mut ranges = set.iter_ranges();
        assert_eq!(Some(0x80..=(char::MAX as u32)), ranges.next());
        assert_eq!(None, ranges.next());
    }

    #[test]
    fn test_unicodeset_span_contains() {
        let ex = vec![0x41, 0x44, 0x46, 0x4B]; // A - D, F - K
        let check = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        assert_eq!(check.span("ABCDE", true), 3);
        assert_eq!(check.span("E", true), 0);
    }

    #[test]
    fn test_unicodeset_span_does_not_contain() {
        let ex = vec![0x41, 0x44, 0x46, 0x4B]; // A - D, F - K
        let check = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        assert_eq!(check.span("DEF", false), 2);
        assert_eq!(check.span("KLMA", false), 3);
    }

    #[test]
    fn test_unicodeset_span_back_contains() {
        let ex = vec![0x41, 0x44, 0x46, 0x4B]; // A - D, F - K
        let check = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        assert_eq!(check.span_back("XYZABFH", true), 3);
        assert_eq!(check.span_back("ABCXYZ", true), 6);
    }

    #[test]
    fn test_unicodeset_span_back_does_not_contain() {
        let ex = vec![0x41, 0x44, 0x46, 0x4B]; // A - D, F - K
        let check = UnicodeSet::from_inversion_list_slice(&ex).unwrap();
        assert_eq!(check.span_back("ABCXYZ", false), 3);
        assert_eq!(check.span_back("XYZABC", false), 6);
    }

    #[test]
    fn test_uniset_to_inv_list() {
        let inv_list: Vec<u32> = vec![
            0x9, 0xE, 0x20, 0x21, 0x85, 0x86, 0xA0, 0xA1, 0x1626, 0x1627, 0x2000, 0x2003, 0x2028,
            0x202A, 0x202F, 0x2030, 0x205F, 0x2060, 0x3000, 0x3001,
        ];
        let inv_list_clone = (&inv_list).clone();
        let s: UnicodeSet = UnicodeSet::from_inversion_list_slice(&inv_list_clone).unwrap();
        let round_trip_inv_list = s.get_inversion_list();
        assert_eq!(round_trip_inv_list, inv_list);
    }

    #[test]
    fn test_serde_serialize() {
        let inv_list = vec![0x41, 0x46, 0x4B, 0x55];
        let uniset = UnicodeSet::from_inversion_list_slice(&inv_list).unwrap();
        let json_str = serde_json::to_string(&uniset).unwrap();
        assert_eq!(json_str, "[65,70,75,85]");
    }

    #[test]
    fn test_serde_deserialize() {
        let inv_list_str = "[65,70,75,85]";
        let exp_inv_list = vec![0x41, 0x46, 0x4B, 0x55];
        let exp_uniset = UnicodeSet::from_inversion_list_slice(&exp_inv_list).unwrap();
        let act_uniset: UnicodeSet = serde_json::from_str(inv_list_str).unwrap();
        assert_eq!(act_uniset, exp_uniset);
    }

    #[test]
    fn test_serde_deserialize_invalid() {
        let inv_list_str = "[65,70,98775,85]";
        let act_result: Result<UnicodeSet, serde_json::Error> = serde_json::from_str(inv_list_str);
        assert!(matches!(act_result, Err(_)));
    }

    #[test]
    fn test_serde_with_postcard_roundtrip() -> Result<(), postcard::Error> {
        let set = UnicodeSet::bmp();
        let set_serialized: Vec<u8> = postcard::to_allocvec(&set).unwrap();
        let set_deserialized: UnicodeSet = postcard::from_bytes::<UnicodeSet>(&set_serialized)?;

        assert_eq!(&set, &set_deserialized);
        assert!(matches!(set_deserialized.inv_list, ZeroVec::Borrowed(_)));

        Ok(())
    }
}
