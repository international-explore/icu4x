// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::EmojiModifierBaseV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::EmojiModifierBaseV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions.with_req(
                    <icu_properties::provider::EmojiModifierBaseV1Marker>::KEY,
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
    &'static <icu_properties::provider::EmojiModifierBaseV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    29u8, 38u8, 0u8, 0u8, 30u8, 38u8, 0u8, 0u8, 249u8, 38u8, 0u8, 0u8, 250u8, 38u8,
                    0u8, 0u8, 10u8, 39u8, 0u8, 0u8, 14u8, 39u8, 0u8, 0u8, 133u8, 243u8, 1u8, 0u8,
                    134u8, 243u8, 1u8, 0u8, 194u8, 243u8, 1u8, 0u8, 197u8, 243u8, 1u8, 0u8, 199u8,
                    243u8, 1u8, 0u8, 200u8, 243u8, 1u8, 0u8, 202u8, 243u8, 1u8, 0u8, 205u8, 243u8,
                    1u8, 0u8, 66u8, 244u8, 1u8, 0u8, 68u8, 244u8, 1u8, 0u8, 70u8, 244u8, 1u8, 0u8,
                    81u8, 244u8, 1u8, 0u8, 102u8, 244u8, 1u8, 0u8, 121u8, 244u8, 1u8, 0u8, 124u8,
                    244u8, 1u8, 0u8, 125u8, 244u8, 1u8, 0u8, 129u8, 244u8, 1u8, 0u8, 132u8, 244u8,
                    1u8, 0u8, 133u8, 244u8, 1u8, 0u8, 136u8, 244u8, 1u8, 0u8, 143u8, 244u8, 1u8,
                    0u8, 144u8, 244u8, 1u8, 0u8, 145u8, 244u8, 1u8, 0u8, 146u8, 244u8, 1u8, 0u8,
                    170u8, 244u8, 1u8, 0u8, 171u8, 244u8, 1u8, 0u8, 116u8, 245u8, 1u8, 0u8, 118u8,
                    245u8, 1u8, 0u8, 122u8, 245u8, 1u8, 0u8, 123u8, 245u8, 1u8, 0u8, 144u8, 245u8,
                    1u8, 0u8, 145u8, 245u8, 1u8, 0u8, 149u8, 245u8, 1u8, 0u8, 151u8, 245u8, 1u8,
                    0u8, 69u8, 246u8, 1u8, 0u8, 72u8, 246u8, 1u8, 0u8, 75u8, 246u8, 1u8, 0u8, 80u8,
                    246u8, 1u8, 0u8, 163u8, 246u8, 1u8, 0u8, 164u8, 246u8, 1u8, 0u8, 180u8, 246u8,
                    1u8, 0u8, 183u8, 246u8, 1u8, 0u8, 192u8, 246u8, 1u8, 0u8, 193u8, 246u8, 1u8,
                    0u8, 204u8, 246u8, 1u8, 0u8, 205u8, 246u8, 1u8, 0u8, 12u8, 249u8, 1u8, 0u8,
                    13u8, 249u8, 1u8, 0u8, 15u8, 249u8, 1u8, 0u8, 16u8, 249u8, 1u8, 0u8, 24u8,
                    249u8, 1u8, 0u8, 32u8, 249u8, 1u8, 0u8, 38u8, 249u8, 1u8, 0u8, 39u8, 249u8,
                    1u8, 0u8, 48u8, 249u8, 1u8, 0u8, 58u8, 249u8, 1u8, 0u8, 60u8, 249u8, 1u8, 0u8,
                    63u8, 249u8, 1u8, 0u8, 119u8, 249u8, 1u8, 0u8, 120u8, 249u8, 1u8, 0u8, 181u8,
                    249u8, 1u8, 0u8, 183u8, 249u8, 1u8, 0u8, 184u8, 249u8, 1u8, 0u8, 186u8, 249u8,
                    1u8, 0u8, 187u8, 249u8, 1u8, 0u8, 188u8, 249u8, 1u8, 0u8, 205u8, 249u8, 1u8,
                    0u8, 208u8, 249u8, 1u8, 0u8, 209u8, 249u8, 1u8, 0u8, 222u8, 249u8, 1u8, 0u8,
                    195u8, 250u8, 1u8, 0u8, 198u8, 250u8, 1u8, 0u8, 240u8, 250u8, 1u8, 0u8, 247u8,
                    250u8, 1u8, 0u8,
                ])
            },
            132usize,
        )
    },
};
