// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::MathV1Marker> for super::super::BakedDataProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::MathV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions
                    .with_req(<icu_properties::provider::MathV1Marker>::KEY, req)
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                value,
            ))),
        })
    }
}
type DataStruct = &'static <icu_properties::provider::MathV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    43u8, 0u8, 0u8, 0u8, 44u8, 0u8, 0u8, 0u8, 60u8, 0u8, 0u8, 0u8, 63u8, 0u8, 0u8,
                    0u8, 94u8, 0u8, 0u8, 0u8, 95u8, 0u8, 0u8, 0u8, 124u8, 0u8, 0u8, 0u8, 125u8,
                    0u8, 0u8, 0u8, 126u8, 0u8, 0u8, 0u8, 127u8, 0u8, 0u8, 0u8, 172u8, 0u8, 0u8,
                    0u8, 173u8, 0u8, 0u8, 0u8, 177u8, 0u8, 0u8, 0u8, 178u8, 0u8, 0u8, 0u8, 215u8,
                    0u8, 0u8, 0u8, 216u8, 0u8, 0u8, 0u8, 247u8, 0u8, 0u8, 0u8, 248u8, 0u8, 0u8,
                    0u8, 208u8, 3u8, 0u8, 0u8, 211u8, 3u8, 0u8, 0u8, 213u8, 3u8, 0u8, 0u8, 214u8,
                    3u8, 0u8, 0u8, 240u8, 3u8, 0u8, 0u8, 242u8, 3u8, 0u8, 0u8, 244u8, 3u8, 0u8,
                    0u8, 247u8, 3u8, 0u8, 0u8, 6u8, 6u8, 0u8, 0u8, 9u8, 6u8, 0u8, 0u8, 22u8, 32u8,
                    0u8, 0u8, 23u8, 32u8, 0u8, 0u8, 50u8, 32u8, 0u8, 0u8, 53u8, 32u8, 0u8, 0u8,
                    64u8, 32u8, 0u8, 0u8, 65u8, 32u8, 0u8, 0u8, 68u8, 32u8, 0u8, 0u8, 69u8, 32u8,
                    0u8, 0u8, 82u8, 32u8, 0u8, 0u8, 83u8, 32u8, 0u8, 0u8, 97u8, 32u8, 0u8, 0u8,
                    101u8, 32u8, 0u8, 0u8, 122u8, 32u8, 0u8, 0u8, 127u8, 32u8, 0u8, 0u8, 138u8,
                    32u8, 0u8, 0u8, 143u8, 32u8, 0u8, 0u8, 208u8, 32u8, 0u8, 0u8, 221u8, 32u8, 0u8,
                    0u8, 225u8, 32u8, 0u8, 0u8, 226u8, 32u8, 0u8, 0u8, 229u8, 32u8, 0u8, 0u8,
                    231u8, 32u8, 0u8, 0u8, 235u8, 32u8, 0u8, 0u8, 240u8, 32u8, 0u8, 0u8, 2u8, 33u8,
                    0u8, 0u8, 3u8, 33u8, 0u8, 0u8, 7u8, 33u8, 0u8, 0u8, 8u8, 33u8, 0u8, 0u8, 10u8,
                    33u8, 0u8, 0u8, 20u8, 33u8, 0u8, 0u8, 21u8, 33u8, 0u8, 0u8, 22u8, 33u8, 0u8,
                    0u8, 24u8, 33u8, 0u8, 0u8, 30u8, 33u8, 0u8, 0u8, 36u8, 33u8, 0u8, 0u8, 37u8,
                    33u8, 0u8, 0u8, 40u8, 33u8, 0u8, 0u8, 42u8, 33u8, 0u8, 0u8, 44u8, 33u8, 0u8,
                    0u8, 46u8, 33u8, 0u8, 0u8, 47u8, 33u8, 0u8, 0u8, 50u8, 33u8, 0u8, 0u8, 51u8,
                    33u8, 0u8, 0u8, 57u8, 33u8, 0u8, 0u8, 60u8, 33u8, 0u8, 0u8, 74u8, 33u8, 0u8,
                    0u8, 75u8, 33u8, 0u8, 0u8, 76u8, 33u8, 0u8, 0u8, 144u8, 33u8, 0u8, 0u8, 168u8,
                    33u8, 0u8, 0u8, 169u8, 33u8, 0u8, 0u8, 175u8, 33u8, 0u8, 0u8, 176u8, 33u8, 0u8,
                    0u8, 178u8, 33u8, 0u8, 0u8, 182u8, 33u8, 0u8, 0u8, 184u8, 33u8, 0u8, 0u8,
                    188u8, 33u8, 0u8, 0u8, 220u8, 33u8, 0u8, 0u8, 221u8, 33u8, 0u8, 0u8, 222u8,
                    33u8, 0u8, 0u8, 228u8, 33u8, 0u8, 0u8, 230u8, 33u8, 0u8, 0u8, 244u8, 33u8, 0u8,
                    0u8, 0u8, 35u8, 0u8, 0u8, 8u8, 35u8, 0u8, 0u8, 12u8, 35u8, 0u8, 0u8, 32u8,
                    35u8, 0u8, 0u8, 34u8, 35u8, 0u8, 0u8, 124u8, 35u8, 0u8, 0u8, 125u8, 35u8, 0u8,
                    0u8, 155u8, 35u8, 0u8, 0u8, 182u8, 35u8, 0u8, 0u8, 183u8, 35u8, 0u8, 0u8,
                    184u8, 35u8, 0u8, 0u8, 208u8, 35u8, 0u8, 0u8, 209u8, 35u8, 0u8, 0u8, 220u8,
                    35u8, 0u8, 0u8, 227u8, 35u8, 0u8, 0u8, 160u8, 37u8, 0u8, 0u8, 162u8, 37u8, 0u8,
                    0u8, 174u8, 37u8, 0u8, 0u8, 184u8, 37u8, 0u8, 0u8, 188u8, 37u8, 0u8, 0u8,
                    194u8, 37u8, 0u8, 0u8, 198u8, 37u8, 0u8, 0u8, 200u8, 37u8, 0u8, 0u8, 202u8,
                    37u8, 0u8, 0u8, 204u8, 37u8, 0u8, 0u8, 207u8, 37u8, 0u8, 0u8, 212u8, 37u8, 0u8,
                    0u8, 226u8, 37u8, 0u8, 0u8, 227u8, 37u8, 0u8, 0u8, 228u8, 37u8, 0u8, 0u8,
                    229u8, 37u8, 0u8, 0u8, 231u8, 37u8, 0u8, 0u8, 237u8, 37u8, 0u8, 0u8, 248u8,
                    37u8, 0u8, 0u8, 0u8, 38u8, 0u8, 0u8, 5u8, 38u8, 0u8, 0u8, 7u8, 38u8, 0u8, 0u8,
                    64u8, 38u8, 0u8, 0u8, 65u8, 38u8, 0u8, 0u8, 66u8, 38u8, 0u8, 0u8, 67u8, 38u8,
                    0u8, 0u8, 96u8, 38u8, 0u8, 0u8, 100u8, 38u8, 0u8, 0u8, 109u8, 38u8, 0u8, 0u8,
                    112u8, 38u8, 0u8, 0u8, 192u8, 39u8, 0u8, 0u8, 0u8, 40u8, 0u8, 0u8, 0u8, 41u8,
                    0u8, 0u8, 0u8, 43u8, 0u8, 0u8, 48u8, 43u8, 0u8, 0u8, 69u8, 43u8, 0u8, 0u8,
                    71u8, 43u8, 0u8, 0u8, 77u8, 43u8, 0u8, 0u8, 41u8, 251u8, 0u8, 0u8, 42u8, 251u8,
                    0u8, 0u8, 97u8, 254u8, 0u8, 0u8, 103u8, 254u8, 0u8, 0u8, 104u8, 254u8, 0u8,
                    0u8, 105u8, 254u8, 0u8, 0u8, 11u8, 255u8, 0u8, 0u8, 12u8, 255u8, 0u8, 0u8,
                    28u8, 255u8, 0u8, 0u8, 31u8, 255u8, 0u8, 0u8, 60u8, 255u8, 0u8, 0u8, 61u8,
                    255u8, 0u8, 0u8, 62u8, 255u8, 0u8, 0u8, 63u8, 255u8, 0u8, 0u8, 92u8, 255u8,
                    0u8, 0u8, 93u8, 255u8, 0u8, 0u8, 94u8, 255u8, 0u8, 0u8, 95u8, 255u8, 0u8, 0u8,
                    226u8, 255u8, 0u8, 0u8, 227u8, 255u8, 0u8, 0u8, 233u8, 255u8, 0u8, 0u8, 237u8,
                    255u8, 0u8, 0u8, 0u8, 212u8, 1u8, 0u8, 85u8, 212u8, 1u8, 0u8, 86u8, 212u8, 1u8,
                    0u8, 157u8, 212u8, 1u8, 0u8, 158u8, 212u8, 1u8, 0u8, 160u8, 212u8, 1u8, 0u8,
                    162u8, 212u8, 1u8, 0u8, 163u8, 212u8, 1u8, 0u8, 165u8, 212u8, 1u8, 0u8, 167u8,
                    212u8, 1u8, 0u8, 169u8, 212u8, 1u8, 0u8, 173u8, 212u8, 1u8, 0u8, 174u8, 212u8,
                    1u8, 0u8, 186u8, 212u8, 1u8, 0u8, 187u8, 212u8, 1u8, 0u8, 188u8, 212u8, 1u8,
                    0u8, 189u8, 212u8, 1u8, 0u8, 196u8, 212u8, 1u8, 0u8, 197u8, 212u8, 1u8, 0u8,
                    6u8, 213u8, 1u8, 0u8, 7u8, 213u8, 1u8, 0u8, 11u8, 213u8, 1u8, 0u8, 13u8, 213u8,
                    1u8, 0u8, 21u8, 213u8, 1u8, 0u8, 22u8, 213u8, 1u8, 0u8, 29u8, 213u8, 1u8, 0u8,
                    30u8, 213u8, 1u8, 0u8, 58u8, 213u8, 1u8, 0u8, 59u8, 213u8, 1u8, 0u8, 63u8,
                    213u8, 1u8, 0u8, 64u8, 213u8, 1u8, 0u8, 69u8, 213u8, 1u8, 0u8, 70u8, 213u8,
                    1u8, 0u8, 71u8, 213u8, 1u8, 0u8, 74u8, 213u8, 1u8, 0u8, 81u8, 213u8, 1u8, 0u8,
                    82u8, 213u8, 1u8, 0u8, 166u8, 214u8, 1u8, 0u8, 168u8, 214u8, 1u8, 0u8, 204u8,
                    215u8, 1u8, 0u8, 206u8, 215u8, 1u8, 0u8, 0u8, 216u8, 1u8, 0u8, 0u8, 238u8, 1u8,
                    0u8, 4u8, 238u8, 1u8, 0u8, 5u8, 238u8, 1u8, 0u8, 32u8, 238u8, 1u8, 0u8, 33u8,
                    238u8, 1u8, 0u8, 35u8, 238u8, 1u8, 0u8, 36u8, 238u8, 1u8, 0u8, 37u8, 238u8,
                    1u8, 0u8, 39u8, 238u8, 1u8, 0u8, 40u8, 238u8, 1u8, 0u8, 41u8, 238u8, 1u8, 0u8,
                    51u8, 238u8, 1u8, 0u8, 52u8, 238u8, 1u8, 0u8, 56u8, 238u8, 1u8, 0u8, 57u8,
                    238u8, 1u8, 0u8, 58u8, 238u8, 1u8, 0u8, 59u8, 238u8, 1u8, 0u8, 60u8, 238u8,
                    1u8, 0u8, 66u8, 238u8, 1u8, 0u8, 67u8, 238u8, 1u8, 0u8, 71u8, 238u8, 1u8, 0u8,
                    72u8, 238u8, 1u8, 0u8, 73u8, 238u8, 1u8, 0u8, 74u8, 238u8, 1u8, 0u8, 75u8,
                    238u8, 1u8, 0u8, 76u8, 238u8, 1u8, 0u8, 77u8, 238u8, 1u8, 0u8, 80u8, 238u8,
                    1u8, 0u8, 81u8, 238u8, 1u8, 0u8, 83u8, 238u8, 1u8, 0u8, 84u8, 238u8, 1u8, 0u8,
                    85u8, 238u8, 1u8, 0u8, 87u8, 238u8, 1u8, 0u8, 88u8, 238u8, 1u8, 0u8, 89u8,
                    238u8, 1u8, 0u8, 90u8, 238u8, 1u8, 0u8, 91u8, 238u8, 1u8, 0u8, 92u8, 238u8,
                    1u8, 0u8, 93u8, 238u8, 1u8, 0u8, 94u8, 238u8, 1u8, 0u8, 95u8, 238u8, 1u8, 0u8,
                    96u8, 238u8, 1u8, 0u8, 97u8, 238u8, 1u8, 0u8, 99u8, 238u8, 1u8, 0u8, 100u8,
                    238u8, 1u8, 0u8, 101u8, 238u8, 1u8, 0u8, 103u8, 238u8, 1u8, 0u8, 107u8, 238u8,
                    1u8, 0u8, 108u8, 238u8, 1u8, 0u8, 115u8, 238u8, 1u8, 0u8, 116u8, 238u8, 1u8,
                    0u8, 120u8, 238u8, 1u8, 0u8, 121u8, 238u8, 1u8, 0u8, 125u8, 238u8, 1u8, 0u8,
                    126u8, 238u8, 1u8, 0u8, 127u8, 238u8, 1u8, 0u8, 128u8, 238u8, 1u8, 0u8, 138u8,
                    238u8, 1u8, 0u8, 139u8, 238u8, 1u8, 0u8, 156u8, 238u8, 1u8, 0u8, 161u8, 238u8,
                    1u8, 0u8, 164u8, 238u8, 1u8, 0u8, 165u8, 238u8, 1u8, 0u8, 170u8, 238u8, 1u8,
                    0u8, 171u8, 238u8, 1u8, 0u8, 188u8, 238u8, 1u8, 0u8, 240u8, 238u8, 1u8, 0u8,
                    242u8, 238u8, 1u8, 0u8,
                ])
            },
            2310usize,
        )
    },
};
