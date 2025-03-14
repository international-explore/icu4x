// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::TerminalPunctuationV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::TerminalPunctuationV1Marker>, DataError>
    {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions.with_req(
                    <icu_properties::provider::TerminalPunctuationV1Marker>::KEY,
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
    &'static <icu_properties::provider::TerminalPunctuationV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    33u8, 0u8, 0u8, 0u8, 34u8, 0u8, 0u8, 0u8, 44u8, 0u8, 0u8, 0u8, 45u8, 0u8, 0u8,
                    0u8, 46u8, 0u8, 0u8, 0u8, 47u8, 0u8, 0u8, 0u8, 58u8, 0u8, 0u8, 0u8, 60u8, 0u8,
                    0u8, 0u8, 63u8, 0u8, 0u8, 0u8, 64u8, 0u8, 0u8, 0u8, 126u8, 3u8, 0u8, 0u8,
                    127u8, 3u8, 0u8, 0u8, 135u8, 3u8, 0u8, 0u8, 136u8, 3u8, 0u8, 0u8, 137u8, 5u8,
                    0u8, 0u8, 138u8, 5u8, 0u8, 0u8, 195u8, 5u8, 0u8, 0u8, 196u8, 5u8, 0u8, 0u8,
                    12u8, 6u8, 0u8, 0u8, 13u8, 6u8, 0u8, 0u8, 27u8, 6u8, 0u8, 0u8, 28u8, 6u8, 0u8,
                    0u8, 29u8, 6u8, 0u8, 0u8, 32u8, 6u8, 0u8, 0u8, 212u8, 6u8, 0u8, 0u8, 213u8,
                    6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 11u8, 7u8, 0u8, 0u8, 12u8, 7u8, 0u8, 0u8,
                    13u8, 7u8, 0u8, 0u8, 248u8, 7u8, 0u8, 0u8, 250u8, 7u8, 0u8, 0u8, 48u8, 8u8,
                    0u8, 0u8, 63u8, 8u8, 0u8, 0u8, 94u8, 8u8, 0u8, 0u8, 95u8, 8u8, 0u8, 0u8, 100u8,
                    9u8, 0u8, 0u8, 102u8, 9u8, 0u8, 0u8, 90u8, 14u8, 0u8, 0u8, 92u8, 14u8, 0u8,
                    0u8, 8u8, 15u8, 0u8, 0u8, 9u8, 15u8, 0u8, 0u8, 13u8, 15u8, 0u8, 0u8, 19u8,
                    15u8, 0u8, 0u8, 74u8, 16u8, 0u8, 0u8, 76u8, 16u8, 0u8, 0u8, 97u8, 19u8, 0u8,
                    0u8, 105u8, 19u8, 0u8, 0u8, 110u8, 22u8, 0u8, 0u8, 111u8, 22u8, 0u8, 0u8,
                    235u8, 22u8, 0u8, 0u8, 238u8, 22u8, 0u8, 0u8, 53u8, 23u8, 0u8, 0u8, 55u8, 23u8,
                    0u8, 0u8, 212u8, 23u8, 0u8, 0u8, 215u8, 23u8, 0u8, 0u8, 218u8, 23u8, 0u8, 0u8,
                    219u8, 23u8, 0u8, 0u8, 2u8, 24u8, 0u8, 0u8, 6u8, 24u8, 0u8, 0u8, 8u8, 24u8,
                    0u8, 0u8, 10u8, 24u8, 0u8, 0u8, 68u8, 25u8, 0u8, 0u8, 70u8, 25u8, 0u8, 0u8,
                    168u8, 26u8, 0u8, 0u8, 172u8, 26u8, 0u8, 0u8, 90u8, 27u8, 0u8, 0u8, 92u8, 27u8,
                    0u8, 0u8, 93u8, 27u8, 0u8, 0u8, 96u8, 27u8, 0u8, 0u8, 125u8, 27u8, 0u8, 0u8,
                    127u8, 27u8, 0u8, 0u8, 59u8, 28u8, 0u8, 0u8, 64u8, 28u8, 0u8, 0u8, 126u8, 28u8,
                    0u8, 0u8, 128u8, 28u8, 0u8, 0u8, 60u8, 32u8, 0u8, 0u8, 62u8, 32u8, 0u8, 0u8,
                    71u8, 32u8, 0u8, 0u8, 74u8, 32u8, 0u8, 0u8, 46u8, 46u8, 0u8, 0u8, 47u8, 46u8,
                    0u8, 0u8, 60u8, 46u8, 0u8, 0u8, 61u8, 46u8, 0u8, 0u8, 65u8, 46u8, 0u8, 0u8,
                    66u8, 46u8, 0u8, 0u8, 76u8, 46u8, 0u8, 0u8, 77u8, 46u8, 0u8, 0u8, 78u8, 46u8,
                    0u8, 0u8, 80u8, 46u8, 0u8, 0u8, 83u8, 46u8, 0u8, 0u8, 85u8, 46u8, 0u8, 0u8,
                    1u8, 48u8, 0u8, 0u8, 3u8, 48u8, 0u8, 0u8, 254u8, 164u8, 0u8, 0u8, 0u8, 165u8,
                    0u8, 0u8, 13u8, 166u8, 0u8, 0u8, 16u8, 166u8, 0u8, 0u8, 243u8, 166u8, 0u8, 0u8,
                    248u8, 166u8, 0u8, 0u8, 118u8, 168u8, 0u8, 0u8, 120u8, 168u8, 0u8, 0u8, 206u8,
                    168u8, 0u8, 0u8, 208u8, 168u8, 0u8, 0u8, 47u8, 169u8, 0u8, 0u8, 48u8, 169u8,
                    0u8, 0u8, 199u8, 169u8, 0u8, 0u8, 202u8, 169u8, 0u8, 0u8, 93u8, 170u8, 0u8,
                    0u8, 96u8, 170u8, 0u8, 0u8, 223u8, 170u8, 0u8, 0u8, 224u8, 170u8, 0u8, 0u8,
                    240u8, 170u8, 0u8, 0u8, 242u8, 170u8, 0u8, 0u8, 235u8, 171u8, 0u8, 0u8, 236u8,
                    171u8, 0u8, 0u8, 80u8, 254u8, 0u8, 0u8, 83u8, 254u8, 0u8, 0u8, 84u8, 254u8,
                    0u8, 0u8, 88u8, 254u8, 0u8, 0u8, 1u8, 255u8, 0u8, 0u8, 2u8, 255u8, 0u8, 0u8,
                    12u8, 255u8, 0u8, 0u8, 13u8, 255u8, 0u8, 0u8, 14u8, 255u8, 0u8, 0u8, 15u8,
                    255u8, 0u8, 0u8, 26u8, 255u8, 0u8, 0u8, 28u8, 255u8, 0u8, 0u8, 31u8, 255u8,
                    0u8, 0u8, 32u8, 255u8, 0u8, 0u8, 97u8, 255u8, 0u8, 0u8, 98u8, 255u8, 0u8, 0u8,
                    100u8, 255u8, 0u8, 0u8, 101u8, 255u8, 0u8, 0u8, 159u8, 3u8, 1u8, 0u8, 160u8,
                    3u8, 1u8, 0u8, 208u8, 3u8, 1u8, 0u8, 209u8, 3u8, 1u8, 0u8, 87u8, 8u8, 1u8, 0u8,
                    88u8, 8u8, 1u8, 0u8, 31u8, 9u8, 1u8, 0u8, 32u8, 9u8, 1u8, 0u8, 86u8, 10u8, 1u8,
                    0u8, 88u8, 10u8, 1u8, 0u8, 240u8, 10u8, 1u8, 0u8, 246u8, 10u8, 1u8, 0u8, 58u8,
                    11u8, 1u8, 0u8, 64u8, 11u8, 1u8, 0u8, 153u8, 11u8, 1u8, 0u8, 157u8, 11u8, 1u8,
                    0u8, 85u8, 15u8, 1u8, 0u8, 90u8, 15u8, 1u8, 0u8, 134u8, 15u8, 1u8, 0u8, 138u8,
                    15u8, 1u8, 0u8, 71u8, 16u8, 1u8, 0u8, 78u8, 16u8, 1u8, 0u8, 190u8, 16u8, 1u8,
                    0u8, 194u8, 16u8, 1u8, 0u8, 65u8, 17u8, 1u8, 0u8, 68u8, 17u8, 1u8, 0u8, 197u8,
                    17u8, 1u8, 0u8, 199u8, 17u8, 1u8, 0u8, 205u8, 17u8, 1u8, 0u8, 206u8, 17u8, 1u8,
                    0u8, 222u8, 17u8, 1u8, 0u8, 224u8, 17u8, 1u8, 0u8, 56u8, 18u8, 1u8, 0u8, 61u8,
                    18u8, 1u8, 0u8, 169u8, 18u8, 1u8, 0u8, 170u8, 18u8, 1u8, 0u8, 75u8, 20u8, 1u8,
                    0u8, 78u8, 20u8, 1u8, 0u8, 90u8, 20u8, 1u8, 0u8, 92u8, 20u8, 1u8, 0u8, 194u8,
                    21u8, 1u8, 0u8, 198u8, 21u8, 1u8, 0u8, 201u8, 21u8, 1u8, 0u8, 216u8, 21u8, 1u8,
                    0u8, 65u8, 22u8, 1u8, 0u8, 67u8, 22u8, 1u8, 0u8, 60u8, 23u8, 1u8, 0u8, 63u8,
                    23u8, 1u8, 0u8, 68u8, 25u8, 1u8, 0u8, 69u8, 25u8, 1u8, 0u8, 70u8, 25u8, 1u8,
                    0u8, 71u8, 25u8, 1u8, 0u8, 66u8, 26u8, 1u8, 0u8, 68u8, 26u8, 1u8, 0u8, 155u8,
                    26u8, 1u8, 0u8, 157u8, 26u8, 1u8, 0u8, 161u8, 26u8, 1u8, 0u8, 163u8, 26u8, 1u8,
                    0u8, 65u8, 28u8, 1u8, 0u8, 68u8, 28u8, 1u8, 0u8, 113u8, 28u8, 1u8, 0u8, 114u8,
                    28u8, 1u8, 0u8, 247u8, 30u8, 1u8, 0u8, 249u8, 30u8, 1u8, 0u8, 112u8, 36u8, 1u8,
                    0u8, 117u8, 36u8, 1u8, 0u8, 110u8, 106u8, 1u8, 0u8, 112u8, 106u8, 1u8, 0u8,
                    245u8, 106u8, 1u8, 0u8, 246u8, 106u8, 1u8, 0u8, 55u8, 107u8, 1u8, 0u8, 58u8,
                    107u8, 1u8, 0u8, 68u8, 107u8, 1u8, 0u8, 69u8, 107u8, 1u8, 0u8, 151u8, 110u8,
                    1u8, 0u8, 153u8, 110u8, 1u8, 0u8, 159u8, 188u8, 1u8, 0u8, 160u8, 188u8, 1u8,
                    0u8, 135u8, 218u8, 1u8, 0u8, 139u8, 218u8, 1u8, 0u8,
                ])
            },
            276usize,
        )
    },
};
