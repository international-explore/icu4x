// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;

#[derive(Copy, Clone, Display, Debug, PartialEq)]
#[non_exhaustive]
pub enum Error {
    #[displaydoc("Could not construct CodePointTrie from deserialized values: {reason}")]
    FromDeserialized { reason: &'static str },
}
