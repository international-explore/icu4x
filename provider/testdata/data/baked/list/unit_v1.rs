// @generated
use icu_provider::prelude::*;
impl ResourceProvider<::icu_list::provider::UnitListV1Marker> for super::super::BakedDataProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<::icu_list::provider::UnitListV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[
            ("ar", AR_AR_EG),
            ("ar-EG", AR_AR_EG),
            ("bn", BN_CCP_UND),
            ("ccp", BN_CCP_UND),
            ("en", EN_EN_001_EN_ZA_FIL),
            ("en-001", EN_EN_001_EN_ZA_FIL),
            ("en-ZA", EN_EN_001_EN_ZA_FIL),
            ("es", ES_ES_AR),
            ("es-AR", ES_ES_AR),
            ("fil", EN_EN_001_EN_ZA_FIL),
            ("fr", FR),
            ("ja", JA),
            ("ru", RU_TR),
            ("sr", SR_SR_CYRL),
            ("sr-Cyrl", SR_SR_CYRL),
            ("sr-Latn", SR_LATN),
            ("th", TH),
            ("tr", RU_TR),
            ("und", BN_CCP_UND),
        ];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions
                    .with_req(<::icu_list::provider::UnitListV1Marker>::KEY, req)
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                value,
            ))),
        })
    }
}
type DataStruct = &'static <::icu_list::provider::UnitListV1Marker as DataMarker>::Yokeable;
static AR_AR_EG: DataStruct = &::icu_list::provider::ListFormatterPatternsV1([
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked("، و", 5u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked("، و", 5u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked("، و", 5u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" و", 3u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked("، و", 5u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked("، و", 5u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked("، و", 5u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" و", 3u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" و", 3u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" و", 3u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" و", 3u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" و", 3u8)
        },
        special_case: None,
    },
]);
static BN_CCP_UND: DataStruct = &::icu_list::provider::ListFormatterPatternsV1([
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
]);
static EN_EN_001_EN_ZA_FIL: DataStruct = &::icu_list::provider::ListFormatterPatternsV1([
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
]);
static ES_ES_AR: DataStruct = &::icu_list::provider::ListFormatterPatternsV1([
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" y ", 3u8)
        },
        special_case: Some(::icu_list::provider::SpecialCasePattern {
            condition: unsafe {
                ::icu_list::provider::StringMatcher::from_dfa_bytes_unchecked(&[
                    114u8, 117u8, 115u8, 116u8, 45u8, 114u8, 101u8, 103u8, 101u8, 120u8, 45u8,
                    97u8, 117u8, 116u8, 111u8, 109u8, 97u8, 116u8, 97u8, 45u8, 100u8, 102u8, 97u8,
                    45u8, 115u8, 112u8, 97u8, 114u8, 115u8, 101u8, 0u8, 0u8, 255u8, 254u8, 0u8,
                    0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 1u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 1u8, 2u8, 2u8, 2u8, 3u8, 4u8, 4u8, 5u8, 6u8, 7u8, 7u8, 7u8, 7u8,
                    7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8,
                    7u8, 7u8, 7u8, 8u8, 9u8, 9u8, 9u8, 10u8, 11u8, 11u8, 12u8, 13u8, 14u8, 14u8,
                    14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8,
                    14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
                    15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 16u8, 16u8, 16u8,
                    16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8,
                    17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8,
                    17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8,
                    17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 18u8, 18u8, 19u8, 19u8, 19u8, 19u8, 19u8,
                    19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8,
                    19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 20u8,
                    21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 22u8,
                    23u8, 23u8, 24u8, 25u8, 25u8, 25u8, 26u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8,
                    27u8, 27u8, 27u8, 27u8, 27u8, 40u8, 1u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 128u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8,
                    0u8, 5u8, 5u8, 6u8, 6u8, 12u8, 12u8, 13u8, 13u8, 0u8, 0u8, 83u8, 0u8, 0u8, 0u8,
                    68u8, 0u8, 0u8, 0u8, 83u8, 0u8, 0u8, 0u8, 68u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 2u8, 0u8, 0u8, 27u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8, 18u8, 0u8, 0u8,
                    0u8, 0u8, 3u8, 0u8, 6u8, 6u8, 13u8, 13u8, 0u8, 0u8, 104u8, 0u8, 0u8, 0u8,
                    104u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 2u8, 2u8,
                    4u8, 7u8, 9u8, 9u8, 11u8, 14u8, 19u8, 19u8, 20u8, 20u8, 21u8, 21u8, 22u8, 22u8,
                    23u8, 23u8, 24u8, 24u8, 25u8, 25u8, 26u8, 26u8, 0u8, 0u8, 68u8, 0u8, 0u8, 0u8,
                    68u8, 0u8, 0u8, 0u8, 68u8, 0u8, 0u8, 0u8, 68u8, 0u8, 0u8, 0u8, 68u8, 0u8, 0u8,
                    0u8, 191u8, 0u8, 0u8, 0u8, 206u8, 0u8, 0u8, 0u8, 221u8, 0u8, 0u8, 0u8, 236u8,
                    0u8, 0u8, 0u8, 221u8, 0u8, 0u8, 0u8, 251u8, 0u8, 0u8, 0u8, 10u8, 1u8, 0u8, 0u8,
                    25u8, 1u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 15u8, 17u8, 0u8, 0u8,
                    68u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 17u8, 17u8, 0u8, 0u8,
                    191u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 15u8, 17u8, 0u8, 0u8,
                    191u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 15u8, 16u8, 0u8, 0u8,
                    191u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 16u8, 17u8, 0u8, 0u8,
                    221u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 15u8, 17u8, 0u8, 0u8,
                    221u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 15u8, 15u8, 0u8, 0u8,
                    221u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 35u8,
                    0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8,
                    18u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8,
                    0u8, 35u8, 0u8, 0u8, 0u8,
                ])
            },
            pattern: unsafe {
                ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" e ", 3u8)
            },
        }),
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" y ", 3u8)
        },
        special_case: Some(::icu_list::provider::SpecialCasePattern {
            condition: unsafe {
                ::icu_list::provider::StringMatcher::from_dfa_bytes_unchecked(&[
                    114u8, 117u8, 115u8, 116u8, 45u8, 114u8, 101u8, 103u8, 101u8, 120u8, 45u8,
                    97u8, 117u8, 116u8, 111u8, 109u8, 97u8, 116u8, 97u8, 45u8, 100u8, 102u8, 97u8,
                    45u8, 115u8, 112u8, 97u8, 114u8, 115u8, 101u8, 0u8, 0u8, 255u8, 254u8, 0u8,
                    0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 1u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 1u8, 2u8, 2u8, 2u8, 3u8, 4u8, 4u8, 5u8, 6u8, 7u8, 7u8, 7u8, 7u8,
                    7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8,
                    7u8, 7u8, 7u8, 8u8, 9u8, 9u8, 9u8, 10u8, 11u8, 11u8, 12u8, 13u8, 14u8, 14u8,
                    14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8,
                    14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
                    15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 16u8, 16u8, 16u8,
                    16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8,
                    17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8,
                    17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8,
                    17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 18u8, 18u8, 19u8, 19u8, 19u8, 19u8, 19u8,
                    19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8,
                    19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 20u8,
                    21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 22u8,
                    23u8, 23u8, 24u8, 25u8, 25u8, 25u8, 26u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8,
                    27u8, 27u8, 27u8, 27u8, 27u8, 40u8, 1u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 128u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8,
                    0u8, 5u8, 5u8, 6u8, 6u8, 12u8, 12u8, 13u8, 13u8, 0u8, 0u8, 83u8, 0u8, 0u8, 0u8,
                    68u8, 0u8, 0u8, 0u8, 83u8, 0u8, 0u8, 0u8, 68u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 2u8, 0u8, 0u8, 27u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8, 18u8, 0u8, 0u8,
                    0u8, 0u8, 3u8, 0u8, 6u8, 6u8, 13u8, 13u8, 0u8, 0u8, 104u8, 0u8, 0u8, 0u8,
                    104u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 2u8, 2u8,
                    4u8, 7u8, 9u8, 9u8, 11u8, 14u8, 19u8, 19u8, 20u8, 20u8, 21u8, 21u8, 22u8, 22u8,
                    23u8, 23u8, 24u8, 24u8, 25u8, 25u8, 26u8, 26u8, 0u8, 0u8, 68u8, 0u8, 0u8, 0u8,
                    68u8, 0u8, 0u8, 0u8, 68u8, 0u8, 0u8, 0u8, 68u8, 0u8, 0u8, 0u8, 68u8, 0u8, 0u8,
                    0u8, 191u8, 0u8, 0u8, 0u8, 206u8, 0u8, 0u8, 0u8, 221u8, 0u8, 0u8, 0u8, 236u8,
                    0u8, 0u8, 0u8, 221u8, 0u8, 0u8, 0u8, 251u8, 0u8, 0u8, 0u8, 10u8, 1u8, 0u8, 0u8,
                    25u8, 1u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 15u8, 17u8, 0u8, 0u8,
                    68u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 17u8, 17u8, 0u8, 0u8,
                    191u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 15u8, 17u8, 0u8, 0u8,
                    191u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 15u8, 16u8, 0u8, 0u8,
                    191u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 16u8, 17u8, 0u8, 0u8,
                    221u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 15u8, 17u8, 0u8, 0u8,
                    221u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 15u8, 15u8, 0u8, 0u8,
                    221u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 35u8,
                    0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8,
                    18u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8,
                    0u8, 35u8, 0u8, 0u8, 0u8,
                ])
            },
            pattern: unsafe {
                ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" e ", 3u8)
            },
        }),
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" y ", 3u8)
        },
        special_case: Some(::icu_list::provider::SpecialCasePattern {
            condition: unsafe {
                ::icu_list::provider::StringMatcher::from_dfa_bytes_unchecked(&[
                    114u8, 117u8, 115u8, 116u8, 45u8, 114u8, 101u8, 103u8, 101u8, 120u8, 45u8,
                    97u8, 117u8, 116u8, 111u8, 109u8, 97u8, 116u8, 97u8, 45u8, 100u8, 102u8, 97u8,
                    45u8, 115u8, 112u8, 97u8, 114u8, 115u8, 101u8, 0u8, 0u8, 255u8, 254u8, 0u8,
                    0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 1u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 1u8, 2u8, 2u8, 2u8, 3u8, 4u8, 4u8, 5u8, 6u8, 7u8, 7u8, 7u8, 7u8,
                    7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8,
                    7u8, 7u8, 7u8, 8u8, 9u8, 9u8, 9u8, 10u8, 11u8, 11u8, 12u8, 13u8, 14u8, 14u8,
                    14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8,
                    14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
                    15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 16u8, 16u8, 16u8,
                    16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 16u8,
                    17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8,
                    17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8,
                    17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 18u8, 18u8, 19u8, 19u8, 19u8, 19u8, 19u8,
                    19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8,
                    19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 20u8,
                    21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 22u8,
                    23u8, 23u8, 24u8, 25u8, 25u8, 25u8, 26u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8,
                    27u8, 27u8, 27u8, 27u8, 27u8, 40u8, 1u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 128u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8,
                    0u8, 5u8, 5u8, 6u8, 6u8, 12u8, 12u8, 13u8, 13u8, 0u8, 0u8, 83u8, 0u8, 0u8, 0u8,
                    68u8, 0u8, 0u8, 0u8, 83u8, 0u8, 0u8, 0u8, 68u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 2u8, 0u8, 0u8, 27u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8, 18u8, 0u8, 0u8,
                    0u8, 0u8, 3u8, 0u8, 6u8, 6u8, 13u8, 13u8, 0u8, 0u8, 104u8, 0u8, 0u8, 0u8,
                    104u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 2u8, 2u8,
                    4u8, 7u8, 9u8, 9u8, 11u8, 14u8, 19u8, 19u8, 20u8, 20u8, 21u8, 21u8, 22u8, 22u8,
                    23u8, 23u8, 24u8, 24u8, 25u8, 25u8, 26u8, 26u8, 0u8, 0u8, 68u8, 0u8, 0u8, 0u8,
                    68u8, 0u8, 0u8, 0u8, 68u8, 0u8, 0u8, 0u8, 68u8, 0u8, 0u8, 0u8, 68u8, 0u8, 0u8,
                    0u8, 191u8, 0u8, 0u8, 0u8, 206u8, 0u8, 0u8, 0u8, 221u8, 0u8, 0u8, 0u8, 236u8,
                    0u8, 0u8, 0u8, 221u8, 0u8, 0u8, 0u8, 251u8, 0u8, 0u8, 0u8, 10u8, 1u8, 0u8, 0u8,
                    25u8, 1u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 15u8, 17u8, 0u8, 0u8,
                    68u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 17u8, 17u8, 0u8, 0u8,
                    191u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 15u8, 17u8, 0u8, 0u8,
                    191u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 15u8, 16u8, 0u8, 0u8,
                    191u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 16u8, 17u8, 0u8, 0u8,
                    221u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 15u8, 17u8, 0u8, 0u8,
                    221u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 15u8, 15u8, 0u8, 0u8,
                    221u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 35u8,
                    0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 18u8, 0u8, 0u8, 0u8,
                    18u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8,
                    0u8, 35u8, 0u8, 0u8, 0u8,
                ])
            },
            pattern: unsafe {
                ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" e ", 3u8)
            },
        }),
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
]);
static FR: DataStruct = &::icu_list::provider::ListFormatterPatternsV1([
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" et ", 4u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" et ", 4u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" et ", 4u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" et ", 4u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
]);
static JA: DataStruct = &::icu_list::provider::ListFormatterPatternsV1([
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked("", 0u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked("", 0u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked("", 0u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked("", 0u8) },
        special_case: None,
    },
]);
static RU_TR: DataStruct = &::icu_list::provider::ListFormatterPatternsV1([
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
]);
static SR_LATN: DataStruct = &::icu_list::provider::ListFormatterPatternsV1([
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" i ", 3u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" i ", 3u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" i ", 3u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" i ", 3u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" i ", 3u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" i ", 3u8)
        },
        special_case: None,
    },
]);
static SR_SR_CYRL: DataStruct = &::icu_list::provider::ListFormatterPatternsV1([
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" и ", 4u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" и ", 4u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" и ", 4u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" и ", 4u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" и ", 4u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" и ", 4u8)
        },
        special_case: None,
    },
]);
static TH: DataStruct = &::icu_list::provider::ListFormatterPatternsV1([
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" และ ", 11u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" และ ", 11u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe {
            ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" และ ", 11u8)
        },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
    ::icu_list::provider::ConditionalListJoinerPattern {
        default: unsafe { ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(" ", 1u8) },
        special_case: None,
    },
]);
