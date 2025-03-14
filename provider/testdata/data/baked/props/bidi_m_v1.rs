// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::BidiMirroredV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::BidiMirroredV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions
                    .with_req(<icu_properties::provider::BidiMirroredV1Marker>::KEY, req)
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                value,
            ))),
        })
    }
}
type DataStruct = &'static <icu_properties::provider::BidiMirroredV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    40u8, 0u8, 0u8, 0u8, 42u8, 0u8, 0u8, 0u8, 60u8, 0u8, 0u8, 0u8, 61u8, 0u8, 0u8,
                    0u8, 62u8, 0u8, 0u8, 0u8, 63u8, 0u8, 0u8, 0u8, 91u8, 0u8, 0u8, 0u8, 92u8, 0u8,
                    0u8, 0u8, 93u8, 0u8, 0u8, 0u8, 94u8, 0u8, 0u8, 0u8, 123u8, 0u8, 0u8, 0u8,
                    124u8, 0u8, 0u8, 0u8, 125u8, 0u8, 0u8, 0u8, 126u8, 0u8, 0u8, 0u8, 171u8, 0u8,
                    0u8, 0u8, 172u8, 0u8, 0u8, 0u8, 187u8, 0u8, 0u8, 0u8, 188u8, 0u8, 0u8, 0u8,
                    58u8, 15u8, 0u8, 0u8, 62u8, 15u8, 0u8, 0u8, 155u8, 22u8, 0u8, 0u8, 157u8, 22u8,
                    0u8, 0u8, 57u8, 32u8, 0u8, 0u8, 59u8, 32u8, 0u8, 0u8, 69u8, 32u8, 0u8, 0u8,
                    71u8, 32u8, 0u8, 0u8, 125u8, 32u8, 0u8, 0u8, 127u8, 32u8, 0u8, 0u8, 141u8,
                    32u8, 0u8, 0u8, 143u8, 32u8, 0u8, 0u8, 64u8, 33u8, 0u8, 0u8, 65u8, 33u8, 0u8,
                    0u8, 1u8, 34u8, 0u8, 0u8, 5u8, 34u8, 0u8, 0u8, 8u8, 34u8, 0u8, 0u8, 14u8, 34u8,
                    0u8, 0u8, 17u8, 34u8, 0u8, 0u8, 18u8, 34u8, 0u8, 0u8, 21u8, 34u8, 0u8, 0u8,
                    23u8, 34u8, 0u8, 0u8, 26u8, 34u8, 0u8, 0u8, 30u8, 34u8, 0u8, 0u8, 31u8, 34u8,
                    0u8, 0u8, 35u8, 34u8, 0u8, 0u8, 36u8, 34u8, 0u8, 0u8, 37u8, 34u8, 0u8, 0u8,
                    38u8, 34u8, 0u8, 0u8, 39u8, 34u8, 0u8, 0u8, 43u8, 34u8, 0u8, 0u8, 52u8, 34u8,
                    0u8, 0u8, 57u8, 34u8, 0u8, 0u8, 58u8, 34u8, 0u8, 0u8, 59u8, 34u8, 0u8, 0u8,
                    77u8, 34u8, 0u8, 0u8, 82u8, 34u8, 0u8, 0u8, 86u8, 34u8, 0u8, 0u8, 95u8, 34u8,
                    0u8, 0u8, 97u8, 34u8, 0u8, 0u8, 98u8, 34u8, 0u8, 0u8, 99u8, 34u8, 0u8, 0u8,
                    100u8, 34u8, 0u8, 0u8, 108u8, 34u8, 0u8, 0u8, 110u8, 34u8, 0u8, 0u8, 141u8,
                    34u8, 0u8, 0u8, 143u8, 34u8, 0u8, 0u8, 147u8, 34u8, 0u8, 0u8, 152u8, 34u8, 0u8,
                    0u8, 153u8, 34u8, 0u8, 0u8, 162u8, 34u8, 0u8, 0u8, 164u8, 34u8, 0u8, 0u8,
                    166u8, 34u8, 0u8, 0u8, 185u8, 34u8, 0u8, 0u8, 190u8, 34u8, 0u8, 0u8, 192u8,
                    34u8, 0u8, 0u8, 201u8, 34u8, 0u8, 0u8, 206u8, 34u8, 0u8, 0u8, 208u8, 34u8, 0u8,
                    0u8, 210u8, 34u8, 0u8, 0u8, 214u8, 34u8, 0u8, 0u8, 238u8, 34u8, 0u8, 0u8,
                    240u8, 34u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 8u8, 35u8, 0u8, 0u8, 12u8, 35u8,
                    0u8, 0u8, 32u8, 35u8, 0u8, 0u8, 34u8, 35u8, 0u8, 0u8, 41u8, 35u8, 0u8, 0u8,
                    43u8, 35u8, 0u8, 0u8, 104u8, 39u8, 0u8, 0u8, 118u8, 39u8, 0u8, 0u8, 192u8,
                    39u8, 0u8, 0u8, 193u8, 39u8, 0u8, 0u8, 195u8, 39u8, 0u8, 0u8, 199u8, 39u8, 0u8,
                    0u8, 200u8, 39u8, 0u8, 0u8, 202u8, 39u8, 0u8, 0u8, 203u8, 39u8, 0u8, 0u8,
                    206u8, 39u8, 0u8, 0u8, 211u8, 39u8, 0u8, 0u8, 215u8, 39u8, 0u8, 0u8, 220u8,
                    39u8, 0u8, 0u8, 223u8, 39u8, 0u8, 0u8, 226u8, 39u8, 0u8, 0u8, 240u8, 39u8, 0u8,
                    0u8, 131u8, 41u8, 0u8, 0u8, 153u8, 41u8, 0u8, 0u8, 155u8, 41u8, 0u8, 0u8,
                    161u8, 41u8, 0u8, 0u8, 162u8, 41u8, 0u8, 0u8, 176u8, 41u8, 0u8, 0u8, 184u8,
                    41u8, 0u8, 0u8, 185u8, 41u8, 0u8, 0u8, 192u8, 41u8, 0u8, 0u8, 198u8, 41u8, 0u8,
                    0u8, 201u8, 41u8, 0u8, 0u8, 202u8, 41u8, 0u8, 0u8, 206u8, 41u8, 0u8, 0u8,
                    211u8, 41u8, 0u8, 0u8, 212u8, 41u8, 0u8, 0u8, 214u8, 41u8, 0u8, 0u8, 216u8,
                    41u8, 0u8, 0u8, 221u8, 41u8, 0u8, 0u8, 225u8, 41u8, 0u8, 0u8, 226u8, 41u8, 0u8,
                    0u8, 227u8, 41u8, 0u8, 0u8, 230u8, 41u8, 0u8, 0u8, 232u8, 41u8, 0u8, 0u8,
                    234u8, 41u8, 0u8, 0u8, 244u8, 41u8, 0u8, 0u8, 250u8, 41u8, 0u8, 0u8, 252u8,
                    41u8, 0u8, 0u8, 254u8, 41u8, 0u8, 0u8, 10u8, 42u8, 0u8, 0u8, 29u8, 42u8, 0u8,
                    0u8, 30u8, 42u8, 0u8, 0u8, 34u8, 42u8, 0u8, 0u8, 36u8, 42u8, 0u8, 0u8, 37u8,
                    42u8, 0u8, 0u8, 38u8, 42u8, 0u8, 0u8, 39u8, 42u8, 0u8, 0u8, 41u8, 42u8, 0u8,
                    0u8, 42u8, 42u8, 0u8, 0u8, 43u8, 42u8, 0u8, 0u8, 47u8, 42u8, 0u8, 0u8, 52u8,
                    42u8, 0u8, 0u8, 54u8, 42u8, 0u8, 0u8, 60u8, 42u8, 0u8, 0u8, 63u8, 42u8, 0u8,
                    0u8, 87u8, 42u8, 0u8, 0u8, 89u8, 42u8, 0u8, 0u8, 100u8, 42u8, 0u8, 0u8, 102u8,
                    42u8, 0u8, 0u8, 106u8, 42u8, 0u8, 0u8, 110u8, 42u8, 0u8, 0u8, 111u8, 42u8, 0u8,
                    0u8, 113u8, 42u8, 0u8, 0u8, 115u8, 42u8, 0u8, 0u8, 117u8, 42u8, 0u8, 0u8,
                    121u8, 42u8, 0u8, 0u8, 164u8, 42u8, 0u8, 0u8, 166u8, 42u8, 0u8, 0u8, 174u8,
                    42u8, 0u8, 0u8, 175u8, 42u8, 0u8, 0u8, 215u8, 42u8, 0u8, 0u8, 220u8, 42u8, 0u8,
                    0u8, 221u8, 42u8, 0u8, 0u8, 222u8, 42u8, 0u8, 0u8, 223u8, 42u8, 0u8, 0u8,
                    226u8, 42u8, 0u8, 0u8, 231u8, 42u8, 0u8, 0u8, 236u8, 42u8, 0u8, 0u8, 239u8,
                    42u8, 0u8, 0u8, 243u8, 42u8, 0u8, 0u8, 244u8, 42u8, 0u8, 0u8, 247u8, 42u8, 0u8,
                    0u8, 252u8, 42u8, 0u8, 0u8, 253u8, 42u8, 0u8, 0u8, 254u8, 42u8, 0u8, 0u8,
                    254u8, 43u8, 0u8, 0u8, 255u8, 43u8, 0u8, 0u8, 2u8, 46u8, 0u8, 0u8, 6u8, 46u8,
                    0u8, 0u8, 9u8, 46u8, 0u8, 0u8, 11u8, 46u8, 0u8, 0u8, 12u8, 46u8, 0u8, 0u8,
                    14u8, 46u8, 0u8, 0u8, 28u8, 46u8, 0u8, 0u8, 30u8, 46u8, 0u8, 0u8, 32u8, 46u8,
                    0u8, 0u8, 42u8, 46u8, 0u8, 0u8, 85u8, 46u8, 0u8, 0u8, 93u8, 46u8, 0u8, 0u8,
                    8u8, 48u8, 0u8, 0u8, 18u8, 48u8, 0u8, 0u8, 20u8, 48u8, 0u8, 0u8, 28u8, 48u8,
                    0u8, 0u8, 89u8, 254u8, 0u8, 0u8, 95u8, 254u8, 0u8, 0u8, 100u8, 254u8, 0u8, 0u8,
                    102u8, 254u8, 0u8, 0u8, 8u8, 255u8, 0u8, 0u8, 10u8, 255u8, 0u8, 0u8, 28u8,
                    255u8, 0u8, 0u8, 29u8, 255u8, 0u8, 0u8, 30u8, 255u8, 0u8, 0u8, 31u8, 255u8,
                    0u8, 0u8, 59u8, 255u8, 0u8, 0u8, 60u8, 255u8, 0u8, 0u8, 61u8, 255u8, 0u8, 0u8,
                    62u8, 255u8, 0u8, 0u8, 91u8, 255u8, 0u8, 0u8, 92u8, 255u8, 0u8, 0u8, 93u8,
                    255u8, 0u8, 0u8, 94u8, 255u8, 0u8, 0u8, 95u8, 255u8, 0u8, 0u8, 97u8, 255u8,
                    0u8, 0u8, 98u8, 255u8, 0u8, 0u8, 100u8, 255u8, 0u8, 0u8, 219u8, 214u8, 1u8,
                    0u8, 220u8, 214u8, 1u8, 0u8, 21u8, 215u8, 1u8, 0u8, 22u8, 215u8, 1u8, 0u8,
                    79u8, 215u8, 1u8, 0u8, 80u8, 215u8, 1u8, 0u8, 137u8, 215u8, 1u8, 0u8, 138u8,
                    215u8, 1u8, 0u8, 195u8, 215u8, 1u8, 0u8, 196u8, 215u8, 1u8, 0u8,
                ])
            },
            553usize,
        )
    },
};
