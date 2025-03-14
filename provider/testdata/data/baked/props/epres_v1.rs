// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::EmojiPresentationV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::EmojiPresentationV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions.with_req(
                    <icu_properties::provider::EmojiPresentationV1Marker>::KEY,
                    req,
                )
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                value,
            ))),
        })
    }
}
type DataStruct =
    &'static <icu_properties::provider::EmojiPresentationV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    26u8, 35u8, 0u8, 0u8, 28u8, 35u8, 0u8, 0u8, 233u8, 35u8, 0u8, 0u8, 237u8, 35u8,
                    0u8, 0u8, 240u8, 35u8, 0u8, 0u8, 241u8, 35u8, 0u8, 0u8, 243u8, 35u8, 0u8, 0u8,
                    244u8, 35u8, 0u8, 0u8, 253u8, 37u8, 0u8, 0u8, 255u8, 37u8, 0u8, 0u8, 20u8,
                    38u8, 0u8, 0u8, 22u8, 38u8, 0u8, 0u8, 72u8, 38u8, 0u8, 0u8, 84u8, 38u8, 0u8,
                    0u8, 127u8, 38u8, 0u8, 0u8, 128u8, 38u8, 0u8, 0u8, 147u8, 38u8, 0u8, 0u8,
                    148u8, 38u8, 0u8, 0u8, 161u8, 38u8, 0u8, 0u8, 162u8, 38u8, 0u8, 0u8, 170u8,
                    38u8, 0u8, 0u8, 172u8, 38u8, 0u8, 0u8, 189u8, 38u8, 0u8, 0u8, 191u8, 38u8, 0u8,
                    0u8, 196u8, 38u8, 0u8, 0u8, 198u8, 38u8, 0u8, 0u8, 206u8, 38u8, 0u8, 0u8,
                    207u8, 38u8, 0u8, 0u8, 212u8, 38u8, 0u8, 0u8, 213u8, 38u8, 0u8, 0u8, 234u8,
                    38u8, 0u8, 0u8, 235u8, 38u8, 0u8, 0u8, 242u8, 38u8, 0u8, 0u8, 244u8, 38u8, 0u8,
                    0u8, 245u8, 38u8, 0u8, 0u8, 246u8, 38u8, 0u8, 0u8, 250u8, 38u8, 0u8, 0u8,
                    251u8, 38u8, 0u8, 0u8, 253u8, 38u8, 0u8, 0u8, 254u8, 38u8, 0u8, 0u8, 5u8, 39u8,
                    0u8, 0u8, 6u8, 39u8, 0u8, 0u8, 10u8, 39u8, 0u8, 0u8, 12u8, 39u8, 0u8, 0u8,
                    40u8, 39u8, 0u8, 0u8, 41u8, 39u8, 0u8, 0u8, 76u8, 39u8, 0u8, 0u8, 77u8, 39u8,
                    0u8, 0u8, 78u8, 39u8, 0u8, 0u8, 79u8, 39u8, 0u8, 0u8, 83u8, 39u8, 0u8, 0u8,
                    86u8, 39u8, 0u8, 0u8, 87u8, 39u8, 0u8, 0u8, 88u8, 39u8, 0u8, 0u8, 149u8, 39u8,
                    0u8, 0u8, 152u8, 39u8, 0u8, 0u8, 176u8, 39u8, 0u8, 0u8, 177u8, 39u8, 0u8, 0u8,
                    191u8, 39u8, 0u8, 0u8, 192u8, 39u8, 0u8, 0u8, 27u8, 43u8, 0u8, 0u8, 29u8, 43u8,
                    0u8, 0u8, 80u8, 43u8, 0u8, 0u8, 81u8, 43u8, 0u8, 0u8, 85u8, 43u8, 0u8, 0u8,
                    86u8, 43u8, 0u8, 0u8, 4u8, 240u8, 1u8, 0u8, 5u8, 240u8, 1u8, 0u8, 207u8, 240u8,
                    1u8, 0u8, 208u8, 240u8, 1u8, 0u8, 142u8, 241u8, 1u8, 0u8, 143u8, 241u8, 1u8,
                    0u8, 145u8, 241u8, 1u8, 0u8, 155u8, 241u8, 1u8, 0u8, 230u8, 241u8, 1u8, 0u8,
                    0u8, 242u8, 1u8, 0u8, 1u8, 242u8, 1u8, 0u8, 2u8, 242u8, 1u8, 0u8, 26u8, 242u8,
                    1u8, 0u8, 27u8, 242u8, 1u8, 0u8, 47u8, 242u8, 1u8, 0u8, 48u8, 242u8, 1u8, 0u8,
                    50u8, 242u8, 1u8, 0u8, 55u8, 242u8, 1u8, 0u8, 56u8, 242u8, 1u8, 0u8, 59u8,
                    242u8, 1u8, 0u8, 80u8, 242u8, 1u8, 0u8, 82u8, 242u8, 1u8, 0u8, 0u8, 243u8, 1u8,
                    0u8, 33u8, 243u8, 1u8, 0u8, 45u8, 243u8, 1u8, 0u8, 54u8, 243u8, 1u8, 0u8, 55u8,
                    243u8, 1u8, 0u8, 125u8, 243u8, 1u8, 0u8, 126u8, 243u8, 1u8, 0u8, 148u8, 243u8,
                    1u8, 0u8, 160u8, 243u8, 1u8, 0u8, 203u8, 243u8, 1u8, 0u8, 207u8, 243u8, 1u8,
                    0u8, 212u8, 243u8, 1u8, 0u8, 224u8, 243u8, 1u8, 0u8, 241u8, 243u8, 1u8, 0u8,
                    244u8, 243u8, 1u8, 0u8, 245u8, 243u8, 1u8, 0u8, 248u8, 243u8, 1u8, 0u8, 63u8,
                    244u8, 1u8, 0u8, 64u8, 244u8, 1u8, 0u8, 65u8, 244u8, 1u8, 0u8, 66u8, 244u8,
                    1u8, 0u8, 253u8, 244u8, 1u8, 0u8, 255u8, 244u8, 1u8, 0u8, 62u8, 245u8, 1u8,
                    0u8, 75u8, 245u8, 1u8, 0u8, 79u8, 245u8, 1u8, 0u8, 80u8, 245u8, 1u8, 0u8,
                    104u8, 245u8, 1u8, 0u8, 122u8, 245u8, 1u8, 0u8, 123u8, 245u8, 1u8, 0u8, 149u8,
                    245u8, 1u8, 0u8, 151u8, 245u8, 1u8, 0u8, 164u8, 245u8, 1u8, 0u8, 165u8, 245u8,
                    1u8, 0u8, 251u8, 245u8, 1u8, 0u8, 80u8, 246u8, 1u8, 0u8, 128u8, 246u8, 1u8,
                    0u8, 198u8, 246u8, 1u8, 0u8, 204u8, 246u8, 1u8, 0u8, 205u8, 246u8, 1u8, 0u8,
                    208u8, 246u8, 1u8, 0u8, 211u8, 246u8, 1u8, 0u8, 213u8, 246u8, 1u8, 0u8, 216u8,
                    246u8, 1u8, 0u8, 221u8, 246u8, 1u8, 0u8, 224u8, 246u8, 1u8, 0u8, 235u8, 246u8,
                    1u8, 0u8, 237u8, 246u8, 1u8, 0u8, 244u8, 246u8, 1u8, 0u8, 253u8, 246u8, 1u8,
                    0u8, 224u8, 247u8, 1u8, 0u8, 236u8, 247u8, 1u8, 0u8, 240u8, 247u8, 1u8, 0u8,
                    241u8, 247u8, 1u8, 0u8, 12u8, 249u8, 1u8, 0u8, 59u8, 249u8, 1u8, 0u8, 60u8,
                    249u8, 1u8, 0u8, 70u8, 249u8, 1u8, 0u8, 71u8, 249u8, 1u8, 0u8, 0u8, 250u8, 1u8,
                    0u8, 112u8, 250u8, 1u8, 0u8, 117u8, 250u8, 1u8, 0u8, 120u8, 250u8, 1u8, 0u8,
                    125u8, 250u8, 1u8, 0u8, 128u8, 250u8, 1u8, 0u8, 135u8, 250u8, 1u8, 0u8, 144u8,
                    250u8, 1u8, 0u8, 173u8, 250u8, 1u8, 0u8, 176u8, 250u8, 1u8, 0u8, 187u8, 250u8,
                    1u8, 0u8, 192u8, 250u8, 1u8, 0u8, 198u8, 250u8, 1u8, 0u8, 208u8, 250u8, 1u8,
                    0u8, 218u8, 250u8, 1u8, 0u8, 224u8, 250u8, 1u8, 0u8, 232u8, 250u8, 1u8, 0u8,
                    240u8, 250u8, 1u8, 0u8, 247u8, 250u8, 1u8, 0u8,
                ])
            },
            1185usize,
        )
    },
};
