// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

struct ErasedCodePointMap16MarkerV1;

use icu_provider::DataMarker;
impl DataMarker for ErasedCodePointMap16MarkerV1 {
    type Yokeable = <icu_properties::provider::ScriptV1Marker as DataMarker>::Yokeable;
}

#[diplomat::bridge]
pub mod ffi {
    use super::ErasedCodePointMap16MarkerV1;
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use icu_properties::maps::{self, CodePointMapResult};
    use icu_provider::prelude::DataPayload;

    #[diplomat::opaque]
    /// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property. For properties whose values fit into 16 bits.
    #[diplomat::rust_link(icu_properties, Mod)]
    pub struct ICU4XCodePointMapData16(DataPayload<ErasedCodePointMap16MarkerV1>);

    pub struct ICU4XCodePointMapData16Response {
        /// The [`ICU4XCodePointMapData16`], if creation was successful.
        pub data: Option<Box<ICU4XCodePointMapData16>>,
        /// Whether creating the [`ICU4XCodePointMapData16`] was successful.
        pub success: bool,
    }

    impl ICU4XCodePointMapData16 {
        /// Gets a map for Unicode property Script from a [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu_properties::maps::get_script, Fn)]
        pub fn try_get_script(provider: &ICU4XDataProvider) -> ICU4XCodePointMapData16Response {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            Self::prepare_result_from_script(maps::get_script(&provider).map(DataPayload::cast))
        }

        fn prepare_result_from_script(
            result: CodePointMapResult<ErasedCodePointMap16MarkerV1>,
        ) -> ICU4XCodePointMapData16Response {
            match result {
                Ok(data) => ICU4XCodePointMapData16Response {
                    data: Some(Box::new(ICU4XCodePointMapData16(data))),
                    success: true,
                },
                Err(_) => ICU4XCodePointMapData16Response {
                    data: None,
                    success: false,
                },
            }
        }

        /// Gets the value for a code point.
        #[diplomat::rust_link(icu_codepointtrie::codepointtrie::CodePointTrie::get_u32, FnInStruct)]
        pub fn get(&self, cp: char) -> u16 {
            self.0.get().code_point_trie.get(cp.into()).0
        }
    }
}
