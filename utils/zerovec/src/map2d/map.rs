// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::AsULE;
use crate::ZeroVec;
use alloc::borrow::Borrow;
use core::cmp::Ordering;
use core::convert::TryFrom;
use core::fmt;
use core::iter::FromIterator;
use core::ops::Range;

use super::ZeroMap2dBorrowed;
use crate::map::ZeroMapKV;
use crate::map::{MutableZeroVecLike, ZeroVecLike};

/// A zero-copy, two-dimensional map datastructure .
///
/// This is an extension of [`ZeroMap`] that supports two layers of keys. For example,
/// to map a pair of an integer and a string to a buffer, you can write:
///
/// ```no_run
/// # use zerovec::ZeroMap2d;
/// let _: ZeroMap2d<u32, str, [u8]> = unimplemented!();
/// ```
///
/// Internally, `ZeroMap2d` stores four zero-copy vectors, one for each type argument plus
/// one more to match between the two vectors of keys.
///
/// # Examples
///
/// ```
/// use zerovec::ZeroMap2d;
///
/// // Example byte buffer representing the map { 1: {2: "three" } }
/// let BINCODE_BYTES: &[u8; 53] = &[
///     2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
///     2, 0, 13, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 116, 104, 114, 101, 101,
/// ];
///
/// // Deserializing to ZeroMap requires no heap allocations.
/// let zero_map: ZeroMap2d<u16, u16, str> =
///     bincode::deserialize(BINCODE_BYTES).expect("Should deserialize successfully");
/// assert_eq!(zero_map.get(&1, &2), Ok("three"));
/// ```
///
/// [`VarZeroVec`]: crate::VarZeroVec
/// [`ZeroMap`]: crate::ZeroMap
// ZeroMap2d contains 4 fields:
//
// - keys0 = sorted list of all K0 in the map
// - joiner = helper vec that maps from a K0 to a range of keys1
// - keys1 = list of all K1 in the map, sorted in ranges for each K0
// - values = list of all values in the map, sorted by (K0, K1)
//
// For a particular K0 at index i, the range of keys1 corresponding to K0 is
// (joiner[i-1]..joiner[i]), where the first range starts at 0.
//
// Required Invariants:
//
// 1. len(keys0) == len(joiner)
// 2. len(keys1) == len(values)
// 3. joiner is sorted
// 4. the last element of joiner is the length of keys1
//
// Optional Invariants:
//
// 5. keys0 is sorted (for binary_search)
// 6. ranges within keys1 are sorted (for binary_search)
// 7. every K0 is associated with at least one K1 (no empty ranges)
//
// During deserialization, these three invariants are not checked, because they put the
// ZeroMap2d in a deterministic state, even though it may have unexpected behavior.
pub struct ZeroMap2d<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    pub(crate) keys0: K0::Container,
    pub(crate) joiner: ZeroVec<'a, u32>,
    pub(crate) keys1: K1::Container,
    pub(crate) values: V::Container,
}

#[derive(PartialEq, Debug)]
/// Used in error types to communicate for which key the error occured.
pub enum KeyError {
    K0,
    K1,
}

impl<'a, K0, K1, V> Default for ZeroMap2d<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K0, K1, V> ZeroMap2d<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    /// Creates a new, empty `ZeroMap2d`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerovec::ZeroMap2d;
    ///
    /// let zm: ZeroMap2d<u16, str, str> = ZeroMap2d::new();
    /// assert!(zm.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            keys0: K0::Container::zvl_new(),
            joiner: ZeroVec::new(),
            keys1: K1::Container::zvl_new(),
            values: V::Container::zvl_new(),
        }
    }

    #[doc(hidden)] // Crabbake internal
    pub const unsafe fn from_parts_unchecked(
        keys0: K0::Container,
        joiner: ZeroVec<'a, u32>,
        keys1: K1::Container,
        values: V::Container,
    ) -> Self {
        Self {
            keys0,
            joiner,
            keys1,
            values,
        }
    }

    /// Construct a new [`ZeroMap2d`] with a given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            keys0: K0::Container::zvl_with_capacity(capacity),
            joiner: ZeroVec::with_capacity(capacity),
            keys1: K1::Container::zvl_with_capacity(capacity),
            values: V::Container::zvl_with_capacity(capacity),
        }
    }

    /// Obtain a borrowed version of this map
    pub fn as_borrowed(&'a self) -> ZeroMap2dBorrowed<'a, K0, K1, V> {
        ZeroMap2dBorrowed {
            keys0: self.keys0.zvl_as_borrowed(),
            joiner: &*self.joiner,
            keys1: self.keys1.zvl_as_borrowed(),
            values: self.values.zvl_as_borrowed(),
        }
    }

    /// The number of values in the [`ZeroMap2d`]
    pub fn len(&self) -> usize {
        self.values.zvl_len()
    }

    /// Whether the [`ZeroMap2d`] is empty
    pub fn is_empty(&self) -> bool {
        self.values.zvl_len() == 0
    }

    /// Remove all elements from the [`ZeroMap2d`]
    pub fn clear(&mut self) {
        self.keys0.zvl_clear();
        self.joiner.clear();
        self.keys1.zvl_clear();
        self.values.zvl_clear();
    }

    /// Reserve capacity for `additional` more elements to be inserted into
    /// the [`ZeroMap2d`] to avoid frequent reallocations.
    ///
    /// See [`Vec::reserve()`](alloc::vec::Vec::reserve) for more information.
    pub fn reserve(&mut self, additional: usize) {
        self.keys0.zvl_reserve(additional);
        self.joiner.zvl_reserve(additional);
        self.keys1.zvl_reserve(additional);
        self.values.zvl_reserve(additional);
    }
}

impl<'a, K0, K1, V> ZeroMap2d<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + Ord,
    K1: ZeroMapKV<'a> + Ord,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    /// Get the value associated with `key0` and `key1`, if it exists.
    ///
    /// ```rust
    /// use zerovec::maps::KeyError;
    /// use zerovec::ZeroMap2d;
    ///
    /// let mut map = ZeroMap2d::new();
    /// map.insert(&1, "one", "foo");
    /// map.insert(&2, "one", "bar");
    /// map.insert(&2, "two", "baz");
    /// assert_eq!(map.get(&1, "one"), Ok("foo"));
    /// assert_eq!(map.get(&1, "two"), Err(KeyError::K1));
    /// assert_eq!(map.get(&2, "one"), Ok("bar"));
    /// assert_eq!(map.get(&2, "two"), Ok("baz"));
    /// assert_eq!(map.get(&3, "three"), Err(KeyError::K0));
    /// ```
    pub fn get(&self, key0: &K0, key1: &K1) -> Result<&V::GetType, KeyError> {
        let (_, range) = self.get_range_for_key0(key0).ok_or(KeyError::K0)?;
        debug_assert!(range.start < range.end); // '<' because every key0 should have a key1
        debug_assert!(range.end <= self.keys1.zvl_len());
        // The above debug_assert! protects the unwrap() below
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        let index = range.start
            + self
                .keys1
                .zvl_binary_search_in_range(key1, range)
                .unwrap()
                .map_err(|_| KeyError::K1)?;
        // This unwrap is protected by the invariant keys1.len() == values.len(),
        // the above debug_assert!, and the contract of zvl_binary_search_in_range.
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        Ok(self.values.zvl_get(index).unwrap())
    }

    /// Returns whether `key0` is contained in this map
    ///
    /// ```rust
    /// use zerovec::ZeroMap2d;
    ///
    /// let mut map = ZeroMap2d::new();
    /// map.insert(&1, "one", "foo");
    /// map.insert(&2, "two", "bar");
    /// assert_eq!(map.contains_key0(&1), true);
    /// assert_eq!(map.contains_key0(&3), false);
    /// ```
    pub fn contains_key0(&self, key0: &K0) -> bool {
        self.keys0.zvl_binary_search(key0).is_ok()
    }

    /// Insert `value` with `key`, returning the existing value if it exists.
    ///
    /// See example in [`Self::get()`].
    pub fn insert(&mut self, key0: &K0, key1: &K1, value: &V) -> Option<V::OwnedType> {
        let (key0_index, range) = self.get_or_insert_range_for_key0(key0);
        debug_assert!(range.start <= range.end); // '<=' because we may have inserted a new key0
        debug_assert!(range.end <= self.keys1.zvl_len());
        // The above debug_assert! protects the unwrap() below
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        let index = range.start
            + match self.keys1.zvl_binary_search_in_range(key1, range).unwrap() {
                Ok(index) => return Some(self.values.zvl_replace(index, value)),
                Err(index) => index,
            };
        self.keys1.zvl_insert(index, key1);
        self.values.zvl_insert(index, value);
        self.joiner_expand(key0_index);
        #[cfg(debug_assertions)]
        self.check_invariants();
        None
    }

    /// Remove the value at `key`, returning it if it exists.
    ///
    /// ```rust
    /// use zerovec::maps::KeyError;
    /// use zerovec::ZeroMap2d;
    ///
    /// let mut map = ZeroMap2d::new();
    /// map.insert(&1, "one", "foo");
    /// map.insert(&2, "two", "bar");
    /// assert_eq!(map.remove(&1, "one"), Ok("foo".to_owned().into_boxed_str()));
    /// assert_eq!(map.get(&1, "one"), Err(KeyError::K0));
    /// assert_eq!(map.remove(&1, "one"), Err(KeyError::K0));
    /// ```
    pub fn remove(&mut self, key0: &K0, key1: &K1) -> Result<V::OwnedType, KeyError> {
        let (key0_index, range) = self.get_range_for_key0(key0).ok_or(KeyError::K0)?;
        debug_assert!(range.start < range.end); // '<' because every key0 should have a key1
        debug_assert!(range.end <= self.keys1.zvl_len());
        // The above debug_assert! protects the unwrap() below
        let is_singleton_range = range.start + 1 == range.end;
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        let index = range.start
            + self
                .keys1
                .zvl_binary_search_in_range(key1, range)
                .unwrap()
                .map_err(|_| KeyError::K1)?;
        self.keys1.zvl_remove(index);
        let removed = self.values.zvl_remove(index);
        self.joiner_shrink(key0_index);
        if is_singleton_range {
            self.remove_key0_index(key0_index);
        }
        #[cfg(debug_assertions)]
        self.check_invariants();
        Ok(removed)
    }

    /// Appends `value` with `key` to the end of the underlying vector, returning
    /// `key` and `value` _if it failed_. Useful for extending with an existing
    /// sorted list.
    /// ```rust
    /// use zerovec::maps::KeyError;
    /// use zerovec::ZeroMap2d;
    ///
    /// let mut map = ZeroMap2d::new();
    /// assert!(map.try_append(&1, "one", "uno").is_none());
    /// assert!(map.try_append(&3, "three", "tres").is_none());
    ///
    /// let unsuccessful = map.try_append(&3, "three", "tres-updated");
    /// assert!(unsuccessful.is_some(), "append duplicate of last key");
    ///
    /// let unsuccessful = map.try_append(&2, "two", "dos");
    /// assert!(unsuccessful.is_some(), "append out of order");
    ///
    /// assert_eq!(map.get(&1, "one"), Ok("uno"));
    ///
    /// // contains the original value for the key: 3
    /// assert_eq!(map.get(&3, "three"), Ok("tres"));
    ///
    /// // not appended since it wasn't in order
    /// assert_eq!(map.get(&2, "two"), Err(KeyError::K0));
    /// ```
    #[must_use]
    pub fn try_append<'b>(
        &mut self,
        key0: &'b K0,
        key1: &'b K1,
        value: &'b V,
    ) -> Option<(&'b K0, &'b K1, &'b V)> {
        if self.is_empty() {
            self.keys0.zvl_push(key0);
            self.joiner.to_mut().push(1u32.to_unaligned());
            self.keys1.zvl_push(key1);
            self.values.zvl_push(value);
            return None;
        }

        // The unwraps are protected by the fact that we are not empty
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        let last_key0 = self.keys0.zvl_get(self.keys0.zvl_len() - 1).unwrap();
        let key0_cmp = K0::Container::t_cmp_get(key0, last_key0);
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        let last_key1 = self.keys1.zvl_get(self.keys1.zvl_len() - 1).unwrap();
        let key1_cmp = K1::Container::t_cmp_get(key1, last_key1);

        // Check for error case (out of order)
        match key0_cmp {
            Ordering::Less => {
                // Error case
                return Some((key0, key1, value));
            }
            Ordering::Equal => {
                match key1_cmp {
                    Ordering::Less | Ordering::Equal => {
                        // Error case
                        return Some((key0, key1, value));
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        let joiner_value = u32::try_from(self.keys1.zvl_len() + 1)
            .expect("Attempted to add more than 2^32 elements to a ZeroMap2d");

        // All OK to append
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        if key0_cmp == Ordering::Greater {
            self.keys0.zvl_push(key0);
            self.joiner.to_mut().push(joiner_value.to_unaligned());
        } else {
            // This unwrap is protected because we are not empty
            *self.joiner.to_mut().last_mut().unwrap() = joiner_value.to_unaligned();
        }
        self.keys1.zvl_push(key1);
        self.values.zvl_push(value);

        #[cfg(debug_assertions)]
        self.check_invariants();

        None
    }

    /// Produce an ordered iterator over keys0.
    pub fn iter_keys0<'b>(&'b self) -> impl Iterator<Item = &'b <K0 as ZeroMapKV<'a>>::GetType> {
        // The unwrap is protected because we are looping over the range of indices in the vec
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        (0..self.keys0.zvl_len()).map(move |idx| self.keys0.zvl_get(idx).unwrap())
    }

    /// Produce an ordered iterator over keys1 for a particular key0, if key0 exists.
    pub fn iter_keys1<'b>(
        &'b self,
        key0: &K0,
    ) -> Option<impl Iterator<Item = &'b <K1 as ZeroMapKV<'a>>::GetType>> {
        let (_, range) = self.get_range_for_key0(key0)?;
        // The unwrap is protected because all ranges are valid according to invariants 1-4
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        Some(range.map(move |idx| self.keys1.zvl_get(idx).unwrap()))
    }

    /// Produce an ordered iterator over keys1 for a particular key0_index, if key0_index exists.
    ///
    /// This method is designed to interoperate with the enumerated key of `iter_keys0`.
    ///
    /// # Panics
    ///
    /// Panics if `key0_index` is out of range.
    ///
    /// # Example
    ///
    /// Loop over all elements of a ZeroMap2d:
    ///
    /// ```
    /// use zerovec::ZeroMap2d;
    ///
    /// let mut map: ZeroMap2d<u16, u16, str> = ZeroMap2d::new();
    /// map.insert(&1, &1, "foo");
    /// map.insert(&2, &3, "bar");
    /// map.insert(&2, &4, "baz");
    ///
    /// let mut total_value = 0;
    ///
    /// let mut values_it = map.iter_values();
    /// for (key0_index, key0) in map.iter_keys0().enumerate() {
    ///     for key1 in map.iter_keys1_by_index(key0_index).unwrap() {
    ///         // This code runs for every (key0, key1) pair
    ///         total_value += key0.as_unsigned_int() as usize;
    ///         total_value += key1.as_unsigned_int() as usize;
    ///         total_value += values_it.next().unwrap().len();
    ///     }
    /// }
    ///
    /// assert_eq!(total_value, 22);
    /// ```
    pub fn iter_keys1_by_index<'b>(
        &'b self,
        key0_index: usize,
    ) -> Option<impl Iterator<Item = &'b <K1 as ZeroMapKV<'a>>::GetType>> {
        assert!(key0_index < self.keys0.zvl_len());
        let range = self.get_range_for_key0_index(key0_index);
        // The unwrap is protected because all ranges are valid according to invariants 1-4
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        Some(range.map(move |idx| self.keys1.zvl_get(idx).unwrap()))
    }

    /// Produce an iterator over values, ordered by the pair (key0,key1)
    pub fn iter_values<'b>(&'b self) -> impl Iterator<Item = &'b <V as ZeroMapKV<'a>>::GetType> {
        // The unwrap is protected because we are looping over the range of indices in the vec
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        (0..self.values.zvl_len()).map(move |idx| self.values.zvl_get(idx).unwrap())
    }

    // INTERNAL ROUTINES FOLLOW //

    /// Given a value that may exist in keys0, returns the corresponding range of keys1
    fn get_range_for_key0(&self, key0: &K0) -> Option<(usize, Range<usize>)> {
        let key0_index = self.keys0.zvl_binary_search(key0).ok()?;
        Some((key0_index, self.get_range_for_key0_index(key0_index)))
    }

    /// Given an index into the joiner array, returns the corresponding range of keys1
    fn get_range_for_key0_index(&self, key0_index: usize) -> Range<usize> {
        debug_assert!(key0_index < self.joiner.len());
        let start = if key0_index == 0 {
            0
        } else {
            // The unwrap is protected by the debug_assert above
            #[allow(clippy::unwrap_used)]
            self.joiner.get(key0_index - 1).unwrap()
        };
        // The unwrap is protected by the debug_assert above
        #[allow(clippy::unwrap_used)]
        let limit = self.joiner.get(key0_index).unwrap();
        (start as usize)..(limit as usize)
    }

    /// Same as `get_range_for_key0`, but creates key0 if it doesn't already exist
    fn get_or_insert_range_for_key0(&mut self, key0: &K0) -> (usize, Range<usize>) {
        match self.keys0.zvl_binary_search(key0) {
            Ok(key0_index) => (key0_index, self.get_range_for_key0_index(key0_index)),
            Err(key0_index) => {
                // Add an entry to self.keys0 and self.joiner
                let joiner_value = if key0_index == 0 {
                    0
                } else {
                    debug_assert!(key0_index <= self.joiner.len());
                    // The unwrap is protected by the debug_assert above and key0_index != 0
                    #[allow(clippy::unwrap_used)]
                    self.joiner.get(key0_index - 1).unwrap()
                };
                self.keys0.zvl_insert(key0_index, key0);
                self.joiner
                    .to_mut()
                    .insert(key0_index, joiner_value.to_unaligned());
                (key0_index, (joiner_value as usize)..(joiner_value as usize))
            }
        }
    }

    /// Removes key0_index from the keys0 array and the joiner array
    fn remove_key0_index(&mut self, key0_index: usize) {
        self.keys0.zvl_remove(key0_index);
        self.joiner.to_mut().remove(key0_index);
    }

    /// Shifts all joiner ranges from key0_index onward one index up
    fn joiner_expand(&mut self, key0_index: usize) {
        #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        self.joiner
            .to_mut()
            .iter_mut()
            .skip(key0_index)
            .for_each(|ref mut v| {
                // TODO(#1410): Make this fallible
                **v = v
                    .as_unsigned_int()
                    .checked_add(1)
                    .expect("Attempted to add more than 2^32 elements to a ZeroMap2d")
                    .to_unaligned()
            });
    }

    /// Shifts all joiner ranges from key0_index onward one index down
    fn joiner_shrink(&mut self, key0_index: usize) {
        #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        self.joiner
            .to_mut()
            .iter_mut()
            .skip(key0_index)
            .for_each(|ref mut v| {
                **v = v
                    .as_unsigned_int()
                    .checked_sub(1)
                    .expect("Shrink should always succeed")
                    .to_unaligned()
            });
    }

    #[cfg(debug_assertions)]
    #[allow(clippy::unwrap_used)] // this is an assertion function
    pub(crate) fn check_invariants(&self) {
        debug_assert_eq!(self.keys0.zvl_len(), self.joiner.len());
        debug_assert_eq!(self.keys1.zvl_len(), self.values.zvl_len());
        debug_assert!(self.keys0.zvl_is_ascending());
        debug_assert!(self.joiner.zvl_is_ascending());
        if let Some(last_joiner) = self.joiner.last() {
            debug_assert_eq!(last_joiner as usize, self.keys1.zvl_len());
        }
        for i in 0..self.joiner.len() {
            let j0 = if i == 0 {
                0
            } else {
                self.joiner.get(i - 1).unwrap() as usize
            };
            let j1 = self.joiner.get(i).unwrap() as usize;
            debug_assert_ne!(j0, j1);
            for j in (j0 + 1)..j1 {
                let m0 = self.keys1.zvl_get(j - 1).unwrap();
                let m1 = self.keys1.zvl_get(j).unwrap();
                debug_assert_eq!(Ordering::Less, K1::Container::get_cmp_get(m0, m1));
            }
        }
    }
}

impl<'a, K0, K1, V> ZeroMap2d<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized + Ord,
    K1: ZeroMapKV<'a> + ?Sized + Ord,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>> + ?Sized,
    V: AsULE + Copy,
{
    /// For cases when `V` is fixed-size, obtain a direct copy of `V` instead of `V::ULE`
    ///
    /// # Examples
    ///
    /// ```
    /// # use zerovec::ZeroMap2d;
    /// let mut map: ZeroMap2d<u16, u16, u16> = ZeroMap2d::new();
    /// map.insert(&1, &2, &3);
    /// map.insert(&1, &4, &5);
    /// map.insert(&6, &7, &8);
    ///
    /// assert_eq!(map.get_copied(&6, &7), Some(8));
    /// ```
    pub fn get_copied(&self, key0: &K0, key1: &K1) -> Option<V> {
        let (_, range) = self.get_range_for_key0(key0)?;
        debug_assert!(range.start < range.end); // '<' because every key0 should have a key1
        debug_assert!(range.end <= self.keys1.zvl_len());
        // The above debug_assert! protects the unwrap() below
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        let index = range.start
            + self
                .keys1
                .zvl_binary_search_in_range(key1, range)
                .unwrap()
                .ok()?;
        self.values.get(index)
    }
}

impl<'a, K0, K1, V> From<ZeroMap2dBorrowed<'a, K0, K1, V>> for ZeroMap2d<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    fn from(other: ZeroMap2dBorrowed<'a, K0, K1, V>) -> Self {
        Self {
            keys0: K0::Container::zvl_from_borrowed(other.keys0),
            joiner: other.joiner.as_zerovec(),
            keys1: K1::Container::zvl_from_borrowed(other.keys1),
            values: V::Container::zvl_from_borrowed(other.values),
        }
    }
}

// We can't use the default PartialEq because ZeroMap2d is invariant
// so otherwise rustc will not automatically allow you to compare ZeroMaps
// with different lifetimes
impl<'a, 'b, K0, K1, V> PartialEq<ZeroMap2d<'b, K0, K1, V>> for ZeroMap2d<'a, K0, K1, V>
where
    K0: for<'c> ZeroMapKV<'c> + ?Sized,
    K1: for<'c> ZeroMapKV<'c> + ?Sized,
    V: for<'c> ZeroMapKV<'c> + ?Sized,
    <K0 as ZeroMapKV<'a>>::Container: PartialEq<<K0 as ZeroMapKV<'b>>::Container>,
    <K1 as ZeroMapKV<'a>>::Container: PartialEq<<K1 as ZeroMapKV<'b>>::Container>,
    <V as ZeroMapKV<'a>>::Container: PartialEq<<V as ZeroMapKV<'b>>::Container>,
{
    fn eq(&self, other: &ZeroMap2d<'b, K0, K1, V>) -> bool {
        self.keys0.eq(&other.keys0)
            && self.joiner.eq(&other.joiner)
            && self.keys1.eq(&other.keys1)
            && self.values.eq(&other.values)
    }
}

impl<'a, K0, K1, V> fmt::Debug for ZeroMap2d<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized,
    K1: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    <K0 as ZeroMapKV<'a>>::Container: fmt::Debug,
    <K1 as ZeroMapKV<'a>>::Container: fmt::Debug,
    <V as ZeroMapKV<'a>>::Container: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("ZeroMap2d")
            .field("keys0", &self.keys0)
            .field("joiner", &self.joiner)
            .field("keys1", &self.keys1)
            .field("values", &self.values)
            .finish()
    }
}

impl<'a, K0, K1, V> Clone for ZeroMap2d<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized,
    K1: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    <K0 as ZeroMapKV<'a>>::Container: Clone,
    <K1 as ZeroMapKV<'a>>::Container: Clone,
    <V as ZeroMapKV<'a>>::Container: Clone,
{
    fn clone(&self) -> Self {
        Self {
            keys0: self.keys0.clone(),
            joiner: self.joiner.clone(),
            keys1: self.keys1.clone(),
            values: self.values.clone(),
        }
    }
}

impl<'a, A, B, C, K0, K1, V> FromIterator<(A, B, C)> for ZeroMap2d<'a, K0, K1, V>
where
    A: Borrow<K0>,
    B: Borrow<K1>,
    C: Borrow<V>,
    K0: ZeroMapKV<'a> + ?Sized + Ord,
    K1: ZeroMapKV<'a> + ?Sized + Ord,
    V: ZeroMapKV<'a> + ?Sized,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (A, B, C)>,
    {
        let iter = iter.into_iter();
        let mut map = match iter.size_hint() {
            (_, Some(upper)) => Self::with_capacity(upper),
            (lower, None) => Self::with_capacity(lower),
        };

        for (key0, key1, value) in iter {
            if let Some((key0, key1, value)) =
                map.try_append(key0.borrow(), key1.borrow(), value.borrow())
            {
                map.insert(key0, key1, value);
            }
        }
        map
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stress_test() {
        let mut zm2d = ZeroMap2d::<u16, str, str>::new();

        assert_eq!(format!("{:?}", zm2d), "ZeroMap2d { keys0: ZeroVec::Borrowed([]), joiner: ZeroVec::Borrowed([]), keys1: [], values: [] }");
        assert_eq!(zm2d.get(&0, ""), Err(KeyError::K0));

        let result = zm2d.try_append(&3, "ccc", "CCC");
        assert!(matches!(result, None));

        assert_eq!(format!("{:?}", zm2d), "ZeroMap2d { keys0: ZeroVec::Owned([3]), joiner: ZeroVec::Owned([1]), keys1: [\"ccc\"], values: [\"CCC\"] }");
        assert_eq!(zm2d.get(&0, ""), Err(KeyError::K0));
        assert_eq!(zm2d.get(&3, ""), Err(KeyError::K1));
        assert_eq!(zm2d.get(&3, "ccc"), Ok("CCC"));
        assert_eq!(zm2d.get(&99, ""), Err(KeyError::K0));

        let result = zm2d.try_append(&3, "eee", "EEE");
        assert!(matches!(result, None));

        assert_eq!(format!("{:?}", zm2d), "ZeroMap2d { keys0: ZeroVec::Owned([3]), joiner: ZeroVec::Owned([2]), keys1: [\"ccc\", \"eee\"], values: [\"CCC\", \"EEE\"] }");
        assert_eq!(zm2d.get(&0, ""), Err(KeyError::K0));
        assert_eq!(zm2d.get(&3, ""), Err(KeyError::K1));
        assert_eq!(zm2d.get(&3, "ccc"), Ok("CCC"));
        assert_eq!(zm2d.get(&3, "eee"), Ok("EEE"));
        assert_eq!(zm2d.get(&3, "five"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&99, ""), Err(KeyError::K0));

        // Out of order
        let result = zm2d.try_append(&3, "ddd", "DD0");
        assert!(matches!(result, Some(_)));

        // Append a few more elements
        let result = zm2d.try_append(&5, "ddd", "DD1");
        assert!(matches!(result, None));
        let result = zm2d.try_append(&7, "ddd", "DD2");
        assert!(matches!(result, None));
        let result = zm2d.try_append(&7, "eee", "EEE");
        assert!(matches!(result, None));
        let result = zm2d.try_append(&7, "www", "WWW");
        assert!(matches!(result, None));
        let result = zm2d.try_append(&9, "yyy", "YYY");
        assert!(matches!(result, None));

        assert_eq!(format!("{:?}", zm2d), "ZeroMap2d { keys0: ZeroVec::Owned([3, 5, 7, 9]), joiner: ZeroVec::Owned([2, 3, 6, 7]), keys1: [\"ccc\", \"eee\", \"ddd\", \"ddd\", \"eee\", \"www\", \"yyy\"], values: [\"CCC\", \"EEE\", \"DD1\", \"DD2\", \"EEE\", \"WWW\", \"YYY\"] }");
        assert_eq!(zm2d.get(&0, ""), Err(KeyError::K0));
        assert_eq!(zm2d.get(&3, ""), Err(KeyError::K1));
        assert_eq!(zm2d.get(&3, "ccc"), Ok("CCC"));
        assert_eq!(zm2d.get(&3, "eee"), Ok("EEE"));
        assert_eq!(zm2d.get(&3, "zzz"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&4, ""), Err(KeyError::K0));
        assert_eq!(zm2d.get(&5, "aaa"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&5, "ddd"), Ok("DD1"));
        assert_eq!(zm2d.get(&5, "zzz"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&6, ""), Err(KeyError::K0));
        assert_eq!(zm2d.get(&7, "aaa"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&7, "ddd"), Ok("DD2"));
        assert_eq!(zm2d.get(&7, "eee"), Ok("EEE"));
        assert_eq!(zm2d.get(&7, "www"), Ok("WWW"));
        assert_eq!(zm2d.get(&7, "yyy"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&7, "zzz"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&8, ""), Err(KeyError::K0));
        assert_eq!(zm2d.get(&9, "aaa"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&9, "www"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&9, "yyy"), Ok("YYY"));
        assert_eq!(zm2d.get(&9, "zzz"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&10, ""), Err(KeyError::K0));
        assert_eq!(zm2d.get(&99, ""), Err(KeyError::K0));

        // Insert some elements
        zm2d.insert(&3, "mmm", "MM0");
        zm2d.insert(&6, "ddd", "DD3");
        zm2d.insert(&6, "mmm", "MM1");
        zm2d.insert(&6, "nnn", "NNN");

        assert_eq!(format!("{:?}", zm2d), "ZeroMap2d { keys0: ZeroVec::Owned([3, 5, 6, 7, 9]), joiner: ZeroVec::Owned([3, 4, 7, 10, 11]), keys1: [\"ccc\", \"eee\", \"mmm\", \"ddd\", \"ddd\", \"mmm\", \"nnn\", \"ddd\", \"eee\", \"www\", \"yyy\"], values: [\"CCC\", \"EEE\", \"MM0\", \"DD1\", \"DD3\", \"MM1\", \"NNN\", \"DD2\", \"EEE\", \"WWW\", \"YYY\"] }");
        assert_eq!(zm2d.get(&0, ""), Err(KeyError::K0));
        assert_eq!(zm2d.get(&3, ""), Err(KeyError::K1));
        assert_eq!(zm2d.get(&3, "ccc"), Ok("CCC"));
        assert_eq!(zm2d.get(&3, "eee"), Ok("EEE"));
        assert_eq!(zm2d.get(&3, "mmm"), Ok("MM0"));
        assert_eq!(zm2d.get(&3, "zzz"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&4, ""), Err(KeyError::K0));
        assert_eq!(zm2d.get(&5, "aaa"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&5, "ddd"), Ok("DD1"));
        assert_eq!(zm2d.get(&5, "zzz"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&6, "aaa"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&6, "ddd"), Ok("DD3"));
        assert_eq!(zm2d.get(&6, "mmm"), Ok("MM1"));
        assert_eq!(zm2d.get(&6, "nnn"), Ok("NNN"));
        assert_eq!(zm2d.get(&6, "zzz"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&7, "aaa"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&7, "ddd"), Ok("DD2"));
        assert_eq!(zm2d.get(&7, "eee"), Ok("EEE"));
        assert_eq!(zm2d.get(&7, "www"), Ok("WWW"));
        assert_eq!(zm2d.get(&7, "yyy"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&7, "zzz"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&8, ""), Err(KeyError::K0));
        assert_eq!(zm2d.get(&9, "aaa"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&9, "www"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&9, "yyy"), Ok("YYY"));
        assert_eq!(zm2d.get(&9, "zzz"), Err(KeyError::K1));
        assert_eq!(zm2d.get(&10, ""), Err(KeyError::K0));
        assert_eq!(zm2d.get(&99, ""), Err(KeyError::K0));

        // Remove some elements
        let result = zm2d.remove(&3, "ccc"); // first element
        assert_eq!(result, Ok(String::from("CCC").into_boxed_str()));
        let result = zm2d.remove(&3, "mmm"); // middle element
        assert_eq!(result, Ok(String::from("MM0").into_boxed_str()));
        let result = zm2d.remove(&5, "ddd"); // singleton K0
        assert_eq!(result, Ok(String::from("DD1").into_boxed_str()));
        let result = zm2d.remove(&9, "yyy"); // last element
        assert_eq!(result, Ok(String::from("YYY").into_boxed_str()));

        assert_eq!(format!("{:?}", zm2d), "ZeroMap2d { keys0: ZeroVec::Owned([3, 6, 7]), joiner: ZeroVec::Owned([1, 4, 7]), keys1: [\"eee\", \"ddd\", \"mmm\", \"nnn\", \"ddd\", \"eee\", \"www\"], values: [\"EEE\", \"DD3\", \"MM1\", \"NNN\", \"DD2\", \"EEE\", \"WWW\"] }");
    }
}
