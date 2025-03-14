// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::borrow::Borrow;
use core::iter::FromIterator;
use litemap::LiteMap;

use super::Key;
use super::Value;

/// A list of [`Key`]-[`Value`] pairs representing functional information
/// about locale's internationnalization preferences.
///
/// Here are examples of fields used in Unicode:
/// - `hc` - Hour Cycle (`h11`, `h12`, `h23`, `h24`)
/// - `ca` - Calendar (`buddhist`, `gregory`, ...)
/// - `fw` - First Day Of the Week (`sun`, `mon`, `sat`, ...)
///
/// You can find the full list in [`Unicode BCP 47 U Extension`] section of LDML.
///
/// [`Unicode BCP 47 U Extension`]: https://unicode.org/reports/tr35/tr35.html#Key_And_Type_Definitions_
///
/// # Examples
///
/// Manually build up a [`Keywords`] object:
///
/// ```
/// use icu::locid::extensions::unicode::{Key, Keywords, Value};
///
/// let key: Key = "hc".parse().expect("Failed to parse a Key.");
/// let value: Value = "h23".parse().expect("Failed to parse a Value.");
/// let keywords: Keywords = vec![(key, value)].into_iter().collect();
///
/// assert_eq!(&keywords.to_string(), "hc-h23");
/// ```
///
/// Access a [`Keywords`] object from a [`Locale`]:
///
/// ```
/// use icu::locid::{unicode_ext_key, unicode_ext_value, Locale};
///
/// let loc: Locale = "und-u-hc-h23-kc-true".parse().expect("Valid BCP-47");
///
/// assert_eq!(
///     loc.extensions.unicode.keywords.get(&unicode_ext_key!("ca")),
///     None
/// );
/// assert_eq!(
///     loc.extensions.unicode.keywords.get(&unicode_ext_key!("hc")),
///     Some(&unicode_ext_value!("h23"))
/// );
/// assert_eq!(
///     loc.extensions.unicode.keywords.get(&unicode_ext_key!("kc")),
///     Some(&unicode_ext_value!("true"))
/// );
///
/// assert_eq!(loc.extensions.unicode.keywords.to_string(), "hc-h23-kc");
/// ```
///
/// [`Locale`]: crate::Locale
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
pub struct Keywords(LiteMap<Key, Value>);

impl Keywords {
    /// Returns a new empty list of key-value pairs. Same as [`default()`](Default::default()), but is `const`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Keywords;
    ///
    /// assert_eq!(Keywords::new(), Keywords::default());
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self(LiteMap::new())
    }

    /// Returns `true` if there are no keywords.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Keywords;
    /// use icu::locid::Locale;
    ///
    /// let loc1 = Locale::from_bytes(b"und-t-h0-hybrid").unwrap();
    /// let loc2 = Locale::from_bytes(b"und-u-ca-buddhist").unwrap();
    ///
    /// assert!(loc1.extensions.unicode.keywords.is_empty());
    /// assert!(!loc2.extensions.unicode.keywords.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the list contains a [`Value`] for the specified [`Key`].
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::{Key, Keywords, Value};
    /// use litemap::LiteMap;
    ///
    /// let key: Key = "ca".parse().expect("Failed to parse a Key.");
    /// let value: Value = "gregory".parse().expect("Failed to parse a Value.");
    /// let keywords: Keywords = vec![(key, value)].into_iter().collect();
    ///
    /// let key: Key = "ca".parse().expect("Failed to parse a Key.");
    /// assert!(&keywords.contains_key(&key));
    /// ```
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Key: Borrow<Q>,
        Q: Ord,
    {
        self.0.contains_key(key)
    }

    /// Returns a reference to the [`Value`] corresponding to the [`Key`].
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::{Key, Keywords, Value};
    ///
    /// let key: Key = "ca".parse().expect("Failed to parse a Key.");
    /// let value: Value = "buddhist".parse().expect("Failed to parse a Value.");
    /// let keywords: Keywords = vec![(key, value)].into_iter().collect();
    ///
    /// let key: Key = "ca".parse().expect("Failed to parse a Key.");
    /// assert_eq!(
    ///     keywords.get(&key).map(|v| v.to_string()),
    ///     Some("buddhist".to_string())
    /// );
    /// ```
    pub fn get<Q>(&self, key: &Q) -> Option<&Value>
    where
        Key: Borrow<Q>,
        Q: Ord,
    {
        self.0.get(key)
    }

    /// Returns a mutable reference to the [`Value`] corresponding to the [`Key`].
    ///
    /// Returns `None` if the key doesn't exist or if the key has no value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::{Key, Keywords, Value};
    ///
    /// let key: Key = "ca".parse().expect("Failed to parse a Key.");
    /// let value: Value = "buddhist".parse().expect("Failed to parse a Value.");
    /// let mut keywords: Keywords = vec![(key, value)].into_iter().collect();
    ///
    /// let key: Key = "ca".parse().expect("Failed to parse a Key.");
    /// if let Some(value) = keywords.get_mut(&key) {
    ///     *value = "gregory".parse().expect("Failed to parse a Value.");
    /// }
    /// assert_eq!(
    ///     keywords.get(&key).map(|v| v.to_string()),
    ///     Some("gregory".to_string())
    /// );
    /// ```
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
    where
        Key: Borrow<Q>,
        Q: Ord,
    {
        self.0.get_mut(key)
    }

    /// Sets the specified keyword, returning the old value if it already existed.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Key;
    /// use icu::locid::extensions::unicode::Value;
    /// use icu::locid::unicode_ext_key;
    /// use icu::locid::Locale;
    /// use std::str::FromStr;
    /// use std::string::ToString;
    ///
    /// const CA_KEY: Key = unicode_ext_key!("ca");
    /// let japanese = Value::from_str("japanese").expect("valid extension subtag");
    /// let buddhist = Value::from_str("buddhist").expect("valid extension subtag");
    ///
    /// let mut loc: Locale = "und-u-hello-ca-buddhist-hc-h12"
    ///     .parse()
    ///     .expect("valid BCP-47 identifier");
    /// let old_value = loc.extensions.unicode.keywords.set(CA_KEY, japanese);
    ///
    /// assert_eq!(old_value, Some(buddhist));
    /// assert_eq!(loc, "und-u-hello-ca-japanese-hc-h12");
    /// ```
    pub fn set(&mut self, key: Key, value: Value) -> Option<Value> {
        self.0.insert(key, value)
    }

    /// Clears all Unicode extension keywords, leaving Unicode attributes.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use std::str::FromStr;
    ///
    /// let mut loc: Locale = "und-u-hello-ca-buddhist-hc-h12".parse().unwrap();
    /// loc.extensions.unicode.keywords.clear();
    /// assert_eq!(loc, "und-u-hello");
    /// ```
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Retains a subset of keywords as specified by the predicate function.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use std::str::FromStr;
    ///
    /// let mut loc: Locale = "und-u-ca-buddhist-hc-h12-ms-metric".parse().unwrap();
    ///
    /// loc.extensions.unicode.keywords.retain_by_key(|k| k == "hc");
    /// assert_eq!(loc, "und-u-hc-h12");
    ///
    /// loc.extensions.unicode.keywords.retain_by_key(|k| k == "ms");
    /// assert_eq!(loc, "und");
    /// ```
    pub fn retain_by_key<F>(&mut self, mut predicate: F)
    where
        F: FnMut(&Key) -> bool,
    {
        self.0.retain(|k, _| predicate(k))
    }

    pub(crate) fn for_each_subtag_str<E, F>(&self, f: &mut F) -> Result<(), E>
    where
        F: FnMut(&str) -> Result<(), E>,
    {
        for (k, v) in self.0.iter() {
            f(k.as_str())?;
            v.for_each_subtag_str(f)?;
        }
        Ok(())
    }

    /// This needs to be its own method to help with type inference in helpers.rs
    #[cfg(test)]
    pub(crate) fn from_tuple_vec(v: Vec<(Key, Value)>) -> Self {
        v.into_iter().collect()
    }
}

impl From<LiteMap<Key, Value>> for Keywords {
    fn from(map: LiteMap<Key, Value>) -> Self {
        Self(map)
    }
}

impl FromIterator<(Key, Value)> for Keywords {
    fn from_iter<I: IntoIterator<Item = (Key, Value)>>(iter: I) -> Self {
        LiteMap::from_iter(iter).into()
    }
}

impl_writeable_for_key_value!(Keywords, "ca", "islamic-civil", "mm", "mm");
