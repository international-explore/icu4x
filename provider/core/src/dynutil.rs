// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Utilities for using trait objects with `DataPayload`.

/// Trait to allow conversion from `DataPayload<T>` to `DataPayload<S>`.
///
/// This trait can be manually implemented in order to enable [`impl_dyn_provider`].
///
/// [`DataPayload::downcast`]: crate::DataPayload::downcast
pub trait UpcastDataPayload<M>
where
    M: crate::DataMarker,
    Self: Sized + crate::DataMarker,
{
    /// Upcast a `DataPayload<T>` to a `DataPayload<S>` where `T` implements trait `S`.
    ///
    /// # Examples
    ///
    /// Upcast and then downcast a data struct of type `Cow<str>` (cart type `String`) via
    /// [`AnyPayload`](crate::any::AnyPayload):
    ///
    /// ```
    /// use icu_provider::dynutil::UpcastDataPayload;
    /// use icu_provider::marker::CowStrMarker;
    /// use icu_provider::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let data = "foo".to_string();
    /// let original = DataPayload::<CowStrMarker>::from_owned(Cow::Owned(data));
    /// let upcasted = AnyMarker::upcast(original);
    /// let downcasted = upcasted
    ///     .downcast::<CowStrMarker>()
    ///     .expect("Type conversion");
    /// assert_eq!(downcasted.get(), "foo");
    /// ```
    fn upcast(other: crate::DataPayload<M>) -> crate::DataPayload<Self>;
}

/// Implements [`DynProvider`] for a marker type `S` on a type that already implements
/// [`DynProvider`] or [`ResourceProvider`] for one or more `M`, where `M` is a concrete type
/// that is convertible to `S` via [`UpcastDataPayload`].
///
/// Use this macro to add support to your data provider for:
///
/// - [`AnyPayload`] if your provider can return typed objects as [`Any`](core::any::Any).
///   Use the shorthand `ANY` as the third argument to the macro.
/// - [`SerializeMarker`] if your provider returns objects implementing [`serde::Serialize`]
///   Use the shorthand `SERDE_SE` as the third argument to the macro.
///
/// ## Wrapping ResourceProvider
///
/// If your type implements [`ResourceProvider`], pass a list of markers as the second argument.
/// This results in a `DynProvider<AnyMarker>` that delegates to a specific marker if the key
/// matches or else returns [`DataErrorKind::MissingResourceKey`].
///
/// ```
/// use icu_provider::datagen::*;
/// use icu_provider::hello_world::*;
/// use icu_provider::inv::InvariantDataProvider;
/// use icu_provider::prelude::*;
///
/// // Example struct that implements ResourceProvider<HelloWorldV1Marker>
/// struct MyProvider;
/// impl ResourceProvider<HelloWorldV1Marker> for MyProvider {
///     fn load_resource(
///         &self,
///         req: &DataRequest,
///     ) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
///         let provider = InvariantDataProvider;
///         provider.load_resource(req)
///     }
/// }
///
/// impl IterableResourceProvider<HelloWorldV1Marker> for MyProvider {
///     fn supported_options(
///         &self,
///     ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
///         Ok(Box::new(core::iter::once(Default::default())))
///     }
/// }
///
/// // Implement DataProvider<AnyMarker> on this struct
/// icu_provider::impl_dyn_provider!(MyProvider, [HelloWorldV1Marker,], ANY);
///
/// let provider = MyProvider;
///
/// // Successful result if the key matches:
/// assert!(matches!(
///     provider.load_payload(HelloWorldV1Marker::KEY, &Default::default()),
///     Ok(_)
/// ));
///
/// // Failure if the key does not match:
/// let DUMMY_KEY = icu_provider::resource_key!("dummy@1");
/// assert!(matches!(
///     provider.load_payload(DUMMY_KEY, &Default::default()),
///     Err(DataError {
///         kind: DataErrorKind::MissingResourceKey,
///         ..
///     })
/// ));
/// ```
///
/// ## Wrapping DynProvider
///
/// It is also possible to wrap a [`DynProvider`] to create another [`DynProvider`]. To do this,
/// pass a match-like statement for keys as the second argument:
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::datagen::*;
/// use icu_provider::hello_world::*;
/// use icu_provider::inv::InvariantDataProvider;
///
/// // Example struct that implements DynProvider<HelloWorldV1Marker>
/// struct MyProvider;
/// impl DynProvider<HelloWorldV1Marker> for MyProvider {
///     fn load_payload(&self, key: ResourceKey, req: &DataRequest)
///             -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
///         let provider = InvariantDataProvider;
///         provider.load_resource(req)
///     }
/// }
///
/// impl IterableDynProvider<HelloWorldV1Marker> for MyProvider {
///     fn supported_options_for_key(&self, _: ResourceKey)
///         -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
///         Ok(Box::new(core::iter::once(Default::default())))
///     }
/// }
///
/// // Implement DataProvider<AnyMarker> on this struct.
/// // Match HelloWorldV1Marker::KEY and delegate to DynProvider<HelloWorldV1Marker>.
/// // Send the wildcard match also to DynProvider<HelloWorldV1Marker>.
/// icu_provider::impl_dyn_provider!(MyProvider, {
///     HelloWorldV1Marker::KEY => HelloWorldV1Marker,
///     _ => HelloWorldV1Marker,
/// }, ANY);
///
/// let provider = MyProvider;
/// let provider = provider.as_any_provider();
///
/// // Successful result if the key matches:
/// assert!(matches!(
///     provider.load_any(HelloWorldV1Marker::KEY, &Default::default()),
///     Ok(_)
/// ));
///
/// // Because of the wildcard, non-matching requests are captured:
/// let DUMMY_KEY = icu_provider::resource_key!("dummy@1");
/// assert!(matches!(
///     provider.load_any(DUMMY_KEY, &Default::default()),
///     Ok(_)
/// ));
/// ```
///
/// [`DynProvider`]: crate::DynProvider
/// [`ResourceProvider`]: crate::ResourceProvider
/// [`AnyPayload`]: (crate::any::AnyPayload)
/// [`DataErrorKind::MissingResourceKey`]: (crate::DataErrorKind::MissingResourceKey)
/// [`SerializeMarker`]: (crate::serde::SerializeMarker)
#[macro_export]
macro_rules! impl_dyn_provider {
    // allow passing in multiple things to do and get dispatched
    ($provider:ty, $arms:tt, $one:tt, $($rest:tt),+) => {
        $crate::impl_dyn_provider!(
            $provider,
            $arms,
            $one
        );

        $crate::impl_dyn_provider!(
            $provider,
            $arms,
            $($rest),+
        );
    };

    // Convenience shortcuts for dyn markers
    ($provider:ty, { $($pat:pat $(if $guard:expr)? => $struct_m:ty),+, }, ANY) => {
        $crate::impl_dyn_provider!(
            $provider,
            { $($pat $(if $guard)? => $struct_m),+, },
            $crate::any::AnyMarker
        );
    };
    ($provider:ty, [ $($struct_m:ty),+, ], ANY) => {
        $crate::impl_dyn_provider!(
            $provider,
            [ $($struct_m),+, ],
            $crate::any::AnyMarker
        );
    };
    ($provider:ty, { $($pat:pat $(if $guard:expr)? => $struct_m:ty),+, }, SERDE_SE) => {
        // If this fails to compile, enable the "datagen" feature on this crate.
        $crate::impl_dyn_provider!(
            $provider,
            { $($pat $(if $guard)? => $struct_m),+, },
            $crate::serde::SerializeMarker
        );
    };
    ($provider:ty, [ $($struct_m:ty),+, ], SERDE_SE) => {
        // If this fails to compile, enable the "datagen" feature on this crate.
        $crate::impl_dyn_provider!(
            $provider,
            [ $($struct_m),+, ],
            $crate::serde::SerializeMarker
        );
    };
    ($provider:ty, { $($pat:pat $(if $guard:expr)? => $struct_m:ty),+, }, CRABBAKE) => {
        // If this fails to compile, enable the "datagen" feature on this crate.
        $crate::impl_dyn_provider!(
            $provider,
            { $($pat $(if $guard)? => $struct_m),+, },
            $crate::datagen::CrabbakeMarker
        );
    };
    ($provider:ty, [ $($struct_m:ty),+, ], CRABBAKE) => {
        // If this fails to compile, enable the "datagen" feature on this crate.
        $crate::impl_dyn_provider!(
            $provider,
            [ $($struct_m),+, ],
            $crate::datagen::CrabbakeMarker
        );
    };

    // DataConverter stuff
    ($provider:ty, [ $($struct_m:ty),+, ], DATA_CONVERTER) => {

         // If this fails to compile, enable the "datagen" feature on this crate.
         $crate::impl_dyn_provider!(
             $provider,
             { $(<$struct_m as $crate::ResourceMarker>::KEY => $struct_m),+, },
             DATA_CONVERTER
         );
    };
    ($provider:ty, { $($pat:pat $(if $guard:expr)? => $struct_m:ty),+, }, DATA_CONVERTER) => {
        // If this fails to compile, enable the "datagen" feature on this crate.
        impl $crate::datagen::DataConverter<$crate::any::AnyMarker, $crate::serde::SerializeMarker> for $provider {
            fn convert(&self, key: $crate::ResourceKey, from: DataPayload<$crate::any::AnyMarker>) -> Result<$crate::DataPayload<$crate::serde::SerializeMarker>, $crate::datagen::ReturnedPayloadError<$crate::any::AnyMarker>> {
                use $crate::datagen::ReturnedPayloadError;
                match key {
                    $(
                        $pat $(if $guard)? => {
                            let result: $crate::DataPayload<$struct_m> = from.downcast().expect("Downcasting failed!");
                            return Ok(result.into_serializable());
                        }
                    )+,
                    _ => Err(ReturnedPayloadError(from, $crate::DataErrorKind::MissingResourceKey.with_key(key)))
                }
            }
        }
        impl $crate::datagen::DataConverter<$crate::buf::BufferMarker, $crate::datagen::HeapStatsMarker> for $provider {
            fn convert(&self, key: $crate::ResourceKey, from: DataPayload<$crate::buf::BufferMarker>) -> Result<$crate::DataPayload<$crate::datagen::HeapStatsMarker>, $crate::datagen::ReturnedPayloadError<$crate::buf::BufferMarker>> {
                use $crate::datagen::ReturnedPayloadError;
                match key {
                    $(
                        $pat $(if $guard)? => {
                            let heap_stats = from.attempt_zero_copy_heap_size::<$struct_m>();
                            return Ok($crate::DataPayload::from_owned(heap_stats));
                        }
                    )+,
                    _ => Err(ReturnedPayloadError(from, $crate::DataErrorKind::MissingResourceKey.with_key(key)))
                }
            }
        }
    };

    ($provider:ty, [ $($struct_m:ty),+, ], ITERABLE_SERDE_SE) => {

         impl $crate::datagen::IterableDynProvider<$crate::serde::SerializeMarker> for $provider
         {
             fn supported_options_for_key(&self, key: $crate::ResourceKey) -> Result<std::boxed::Box<dyn Iterator<Item = $crate::ResourceOptions> + '_>, $crate::DataError> {
                 match key {
                     $(
                         <$struct_m as $crate::ResourceMarker>::KEY => {
                             $crate::datagen::IterableResourceProvider::<$struct_m>::supported_options(self)
                         }
                     )+,
                     _ => Err($crate::DataErrorKind::MissingResourceKey.with_key(key))
                 }
             }
         }
    };
    ($provider:ty, { $($pat:pat $(if $guard:expr)? => $struct_m:ty),+, }, ITERABLE_SERDE_SE) => {

        impl $crate::datagen::IterableDynProvider<$crate::serde::SerializeMarker> for $provider {
            fn supported_options_for_key(&self, key: $crate::ResourceKey) -> Result<std::boxed::Box<dyn Iterator<Item = $crate::ResourceOptions> + '_>, $crate::DataError> {
                match key {
                    $(
                        $pat $(if $guard)? => {
                            $crate::datagen::IterableDynProvider::<$struct_m>::supported_options_for_key(self, key)
                        }
                    )+,
                    _ => Err($crate::DataErrorKind::MissingResourceKey.with_key(key))
                }
            }
        }
    };

    ($provider:ty, [ $($struct_m:ty),+, ], ITERABLE_CRABBAKE) => {

         impl $crate::datagen::IterableDynProvider<$crate::datagen::CrabbakeMarker> for $provider
         {
             fn supported_options_for_key(&self, key: $crate::ResourceKey) -> Result<Box<dyn Iterator<Item = $crate::ResourceOptions> + '_>, $crate::DataError> {
                 match key {
                     $(
                         <$struct_m as $crate::ResourceMarker>::KEY => {
                             $crate::datagen::IterableResourceProvider::<$struct_m>::supported_options(self)
                         }
                     )+,
                     _ => Err($crate::DataErrorKind::MissingResourceKey.with_key(key))
                 }
             }
         }
    };
    ($provider:ty, { $($pat:pat $(if $guard:expr)? => $struct_m:ty),+, }, ITERABLE_CRABBAKE) => {

        impl $crate::datagen::IterableDynProvider<$crate::datagen::CrabbakeMarker> for $provider {
            fn supported_options_for_key(&self, key: $crate::ResourceKey) -> Result<Box<dyn Iterator<Item = $crate::ResourceOptions> + '_>, $crate::DataError> {
                match key {
                    $(
                        $pat $(if $guard)? => {
                            $crate::datagen::IterableDynProvider::<$struct_m>::supported_options_for_key(self, key)
                        }
                    )+,
                    _ => Err($crate::DataErrorKind::MissingResourceKey.with_key(key))
                }
            }
        }
    };

    // DynProvider stuff
    ($provider:ty, { $($pat:pat $(if $guard:expr)? => $struct_m:ty),+, }, $dyn_m:path) => {
        impl $crate::DynProvider<$dyn_m> for $provider
        {
            fn load_payload(
                &self,
                key: $crate::ResourceKey,
                req: &$crate::DataRequest,
            ) -> Result<
                $crate::DataResponse<$dyn_m>,
                $crate::DataError,
            > {
                match key {
                    $(
                        $pat $(if $guard)? => {
                            let result: $crate::DataResponse<$struct_m> =
                                $crate::DynProvider::<$struct_m>::load_payload(self, key, req)?;
                            Ok(DataResponse {
                                metadata: result.metadata,
                                payload: result.payload.map(|p| {
                                    $crate::dynutil::UpcastDataPayload::<$struct_m>::upcast(p)
                                }),
                            })
                        }
                    )+,
                    _ => Err($crate::DataErrorKind::MissingResourceKey.with_req(key, req))
                }
            }
        }

    };
    ($provider:ty, [ $($struct_m:ty),+, ], $dyn_m:path) => {
        impl $crate::DynProvider<$dyn_m> for $provider
        {
            fn load_payload(
                &self,
                key: $crate::ResourceKey,
                req: &$crate::DataRequest,
            ) -> Result<
                $crate::DataResponse<$dyn_m>,
                $crate::DataError,
            > {
                match key {
                    $(
                        <$struct_m as $crate::ResourceMarker>::KEY => {
                            let result: $crate::DataResponse<$struct_m> =
                                $crate::ResourceProvider::load_resource(self, req)?;
                            Ok(DataResponse {
                                metadata: result.metadata,
                                payload: result.payload.map(|p| {
                                    $crate::dynutil::UpcastDataPayload::<$struct_m>::upcast(p)
                                }),
                            })
                        }
                    )+,
                    _ => Err($crate::DataErrorKind::MissingResourceKey.with_req(key, req))
                }
            }
        }
    };
}
