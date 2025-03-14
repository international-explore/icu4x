// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Locale-invariant data provider that requires no I/O.

#[cfg(feature = "datagen")]
use crate::datagen::IterableDynProvider;
use crate::prelude::*;
#[cfg(feature = "datagen")]
use alloc::boxed::Box;

/// A locale-invariant data provider. Sometimes useful for testing. Not intended to be used in
/// production environments.
///
/// The objects returned by [`InvariantDataProvider`] are guaranteed to conform to the correct struct
/// definition, so [`InvariantDataProvider`] can also be used to validate unknown data providers.
///
/// # Examples
///
/// ```
/// use icu_provider::hello_world::HelloWorldV1Marker;
/// use icu_provider::inv::InvariantDataProvider;
/// use icu_provider::prelude::*;
/// use std::borrow::Cow;
///
/// let provider = InvariantDataProvider;
/// let result: DataPayload<HelloWorldV1Marker> = provider
///     .load_resource(&DataRequest::default())
///     .unwrap()
///     .take_payload()
///     .unwrap();
///
/// assert_eq!("(und) Hello World", result.get().message);
/// ```
#[allow(clippy::exhaustive_structs)] // stable type
pub struct InvariantDataProvider;

impl<M> ResourceProvider<M> for InvariantDataProvider
where
    M: ResourceMarker,
    M::Yokeable: Default,
{
    fn load_resource(&self, _: &DataRequest) -> Result<DataResponse<M>, DataError> {
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(M::Yokeable::default())),
        })
    }
}

impl<M> DynProvider<M> for InvariantDataProvider
where
    M: DataMarker,
    M::Yokeable: Default,
{
    fn load_payload(&self, _: ResourceKey, _: &DataRequest) -> Result<DataResponse<M>, DataError> {
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(M::Yokeable::default())),
        })
    }
}

#[cfg(feature = "datagen")]
impl<M> IterableDynProvider<M> for InvariantDataProvider
where
    M: DataMarker,
    M::Yokeable: Default,
{
    fn supported_options_for_key(
        &self,
        _: ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        Ok(Box::new(core::iter::once(ResourceOptions::default())))
    }
}
