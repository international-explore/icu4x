// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! See [`VarZeroVec`](crate::VarZeroVec) for details

pub(crate) mod components;
pub(crate) mod owned;
pub(crate) mod slice;
pub(crate) mod vec;

#[cfg(feature = "crabbake")]
mod crabbake;

#[cfg(feature = "serde")]
mod serde;

pub use crate::{VarZeroSlice, VarZeroVec};

#[cfg(feature = "bench")]
#[doc(hidden)]
pub use components::VarZeroVecComponents;

pub use owned::VarZeroVecOwned;
