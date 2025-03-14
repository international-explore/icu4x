// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use alloc::boxed::Box;
use icu_provider::prelude::*;

use icu_locid::LanguageIdentifier;

impl<D, F> RequestFilterDataProvider<D, F>
where
    F: Fn(&DataRequest) -> bool + Sync,
{
    /// Filter out data requests with certain langids according to the predicate function. The
    /// predicate should return `true` to allow a langid and `false` to reject a langid.
    ///
    /// Data requests with no langid will be allowed. To reject data requests without a langid,
    /// chain this with [`Self::require_langid`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::LanguageIdentifier;
    /// use icu_locid::{langid, language, locale};
    /// use icu_provider::datagen::*;
    /// use icu_provider::hello_world::*;
    /// use icu_provider::prelude::*;
    /// use icu_provider_adapters::filter::Filterable;
    ///
    /// let provider = HelloWorldProvider::new_with_placeholder_data()
    ///     .filterable("Demo no-English filter")
    ///     .filter_by_langid(|langid| langid.language != language!("en"));
    ///
    /// // German requests should succeed:
    /// let req_de = DataRequest {
    ///     options: locale!("de").into(),
    ///     metadata: Default::default(),
    /// };
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> = provider.load_resource(&req_de);
    /// assert!(matches!(response, Ok(_)));
    ///
    /// // English requests should fail:
    /// let req_en = DataRequest {
    ///     options: locale!("en-US").into(),
    ///     metadata: Default::default(),
    /// };
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> = provider.load_resource(&req_en);
    /// assert!(matches!(
    ///     response,
    ///     Err(DataError {
    ///         kind: DataErrorKind::FilteredResource,
    ///         ..
    ///     })
    /// ));
    ///
    /// // English should not appear in the iterator result:
    /// let supported_langids = provider
    ///     .supported_options()
    ///     .expect("Should successfully make an iterator of supported locales")
    ///     .map(|options| options.get_langid())
    ///     .collect::<Vec<LanguageIdentifier>>();
    /// assert!(supported_langids.contains(&langid!("de")));
    /// assert!(!supported_langids.contains(&langid!("en")));
    /// ```
    pub fn filter_by_langid<'a>(
        self,
        predicate: impl Fn(&LanguageIdentifier) -> bool + Sync + 'a,
    ) -> RequestFilterDataProvider<D, Box<dyn Fn(&DataRequest) -> bool + Sync + 'a>>
    where
        F: 'a,
    {
        let old_predicate = self.predicate;
        RequestFilterDataProvider {
            inner: self.inner,
            predicate: Box::new(move |request| -> bool {
                if !(old_predicate)(request) {
                    return false;
                }
                predicate(&request.options.get_langid())
            }),
            filter_name: self.filter_name,
        }
    }

    /// Filter out data request except those having a language identifier that exactly matches
    /// one in the allowlist.
    ///
    /// This will be replaced with a smarter algorithm for locale filtering; see
    /// <https://github.com/unicode-org/icu4x/issues/834>
    ///
    /// Data requests with no langid will be allowed. To reject data requests without a langid,
    /// chain this with [`Self::require_langid`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::{langid, locale};
    /// use icu_provider::hello_world::*;
    /// use icu_provider::prelude::*;
    /// use icu_provider_adapters::filter::Filterable;
    ///
    /// let allowlist = vec![langid!("de"), langid!("zh")];
    /// let provider = HelloWorldProvider::new_with_placeholder_data()
    ///     .filterable("Demo German+Chinese filter")
    ///     .filter_by_langid_allowlist_strict(&allowlist);
    ///
    /// // German requests should succeed:
    /// let req_de = DataRequest {
    ///     options: locale!("de").into(),
    ///     metadata: Default::default(),
    /// };
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> = provider.load_resource(&req_de);
    /// assert!(matches!(response, Ok(_)));
    ///
    /// // English requests should fail:
    /// let req_en = DataRequest {
    ///     options: locale!("en-US").into(),
    ///     metadata: Default::default(),
    /// };
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> = provider.load_resource(&req_en);
    /// assert!(matches!(
    ///     response,
    ///     Err(DataError {
    ///         kind: DataErrorKind::FilteredResource,
    ///         ..
    ///     })
    /// ));
    /// assert_eq!(
    ///     response.unwrap_err().str_context,
    ///     Some("Demo German+Chinese filter")
    /// );
    /// ```
    pub fn filter_by_langid_allowlist_strict<'a>(
        self,
        allowlist: &'a [LanguageIdentifier],
    ) -> RequestFilterDataProvider<D, Box<dyn Fn(&DataRequest) -> bool + Sync + 'a>>
    where
        F: 'a,
    {
        let old_predicate = self.predicate;
        RequestFilterDataProvider {
            inner: self.inner,
            predicate: Box::new(move |request| -> bool {
                if !(old_predicate)(request) {
                    return false;
                }
                request.options.is_langid_und() || allowlist.contains(&request.options.get_langid())
            }),
            filter_name: self.filter_name,
        }
    }

    /// Require that data requests contain a langid.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::locale;
    /// use icu_provider::hello_world::*;
    /// use icu_provider::prelude::*;
    /// use icu_provider_adapters::filter::Filterable;
    ///
    /// let provider = HelloWorldProvider::new_with_placeholder_data()
    ///     .filterable("Demo require-langid filter")
    ///     .require_langid();
    ///
    /// // Requests with a langid should succeed:
    /// let req_with_langid = DataRequest {
    ///     options: locale!("de").into(),
    ///     metadata: Default::default(),
    /// };
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> =
    ///     provider.load_resource(&req_with_langid);
    /// assert!(matches!(response, Ok(_)));
    ///
    /// // Requests without a langid should fail:
    /// let req_no_langid = DataRequest {
    ///     options: Default::default(),
    ///     metadata: Default::default(),
    /// };
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> =
    ///     provider.load_resource(&req_no_langid);
    /// assert!(matches!(
    ///     response,
    ///     Err(DataError {
    ///         kind: DataErrorKind::FilteredResource,
    ///         ..
    ///     })
    /// ));
    /// ```
    pub fn require_langid<'a>(
        self,
    ) -> RequestFilterDataProvider<D, Box<dyn Fn(&DataRequest) -> bool + Sync + 'a>>
    where
        F: 'a,
    {
        let old_predicate = self.predicate;
        RequestFilterDataProvider {
            inner: self.inner,
            predicate: Box::new(move |request| -> bool {
                if !(old_predicate)(request) {
                    return false;
                }
                !request.options.is_langid_und()
            }),
            filter_name: self.filter_name,
        }
    }
}
