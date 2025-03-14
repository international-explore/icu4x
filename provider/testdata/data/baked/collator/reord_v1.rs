// @generated
use icu_provider::prelude::*;
impl ResourceProvider<::icu_collator::provider::CollationReorderingV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<::icu_collator::provider::CollationReorderingV1Marker>, DataError>
    {
        static VALUES: &[(&str, DataStruct)] = &[("bn", BN), ("ja", JA), ("th", TH)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions.with_req(
                    <::icu_collator::provider::CollationReorderingV1Marker>::KEY,
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
    &'static <::icu_collator::provider::CollationReorderingV1Marker as DataMarker>::Yokeable;
static BN: DataStruct = &::icu_collator::provider::CollationReorderingV1 {
    min_high_no_reorder: 1906311168u32,
    reorder_table: unsafe {
        ::zerovec::ZeroVec::from_bytes_unchecked(&[
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
            16u8, 17u8, 18u8, 19u8, 20u8, 21u8, 22u8, 23u8, 24u8, 25u8, 26u8, 27u8, 28u8, 29u8,
            30u8, 31u8, 32u8, 33u8, 34u8, 35u8, 36u8, 37u8, 38u8, 39u8, 40u8, 49u8, 50u8, 51u8,
            52u8, 53u8, 54u8, 55u8, 56u8, 57u8, 58u8, 59u8, 60u8, 61u8, 62u8, 63u8, 64u8, 65u8,
            66u8, 67u8, 68u8, 69u8, 70u8, 71u8, 72u8, 73u8, 74u8, 75u8, 76u8, 77u8, 78u8, 79u8,
            80u8, 81u8, 82u8, 83u8, 84u8, 85u8, 86u8, 87u8, 88u8, 89u8, 90u8, 91u8, 92u8, 93u8,
            94u8, 95u8, 96u8, 97u8, 98u8, 99u8, 100u8, 101u8, 102u8, 103u8, 102u8, 103u8, 104u8,
            105u8, 106u8, 107u8, 108u8, 109u8, 40u8, 39u8, 41u8, 42u8, 43u8, 44u8, 45u8, 46u8,
            47u8, 0u8, 114u8, 115u8, 116u8, 117u8, 118u8, 119u8, 120u8, 121u8, 122u8, 123u8, 124u8,
            125u8, 126u8, 127u8, 128u8, 129u8, 130u8, 131u8, 132u8, 133u8, 134u8, 135u8, 136u8,
            137u8, 138u8, 139u8, 140u8, 141u8, 142u8, 143u8, 144u8, 145u8, 146u8, 147u8, 148u8,
            149u8, 150u8, 151u8, 152u8, 153u8, 154u8, 155u8, 156u8, 157u8, 158u8, 159u8, 160u8,
            161u8, 162u8, 163u8, 164u8, 165u8, 166u8, 167u8, 168u8, 169u8, 170u8, 171u8, 172u8,
            173u8, 174u8, 175u8, 176u8, 177u8, 178u8, 179u8, 180u8, 181u8, 182u8, 183u8, 184u8,
            185u8, 186u8, 187u8, 188u8, 189u8, 190u8, 191u8, 192u8, 193u8, 194u8, 195u8, 196u8,
            197u8, 198u8, 199u8, 200u8, 201u8, 202u8, 203u8, 204u8, 205u8, 206u8, 207u8, 208u8,
            209u8, 210u8, 211u8, 212u8, 213u8, 214u8, 215u8, 216u8, 217u8, 218u8, 219u8, 220u8,
            221u8, 222u8, 223u8, 224u8, 225u8, 226u8, 227u8, 228u8, 229u8, 230u8, 231u8, 232u8,
            233u8, 234u8, 235u8, 236u8, 237u8, 238u8, 239u8, 240u8, 241u8, 242u8, 243u8, 244u8,
            245u8, 246u8, 247u8, 248u8, 249u8, 250u8, 251u8, 252u8, 253u8, 254u8, 255u8,
        ])
    },
    reorder_ranges: unsafe {
        ::zerovec::ZeroVec::from_bytes_unchecked(&[191u8, 255u8, 160u8, 113u8])
    },
};
static JA: DataStruct = &::icu_collator::provider::CollationReorderingV1 {
    min_high_no_reorder: 4261412864u32,
    reorder_table: unsafe {
        ::zerovec::ZeroVec::from_bytes_unchecked(&[
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
            16u8, 17u8, 18u8, 19u8, 20u8, 21u8, 22u8, 23u8, 24u8, 25u8, 26u8, 27u8, 28u8, 29u8,
            30u8, 31u8, 32u8, 33u8, 34u8, 35u8, 36u8, 37u8, 38u8, 39u8, 40u8, 41u8, 42u8, 43u8,
            44u8, 45u8, 46u8, 47u8, 48u8, 49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8, 57u8,
            58u8, 59u8, 60u8, 61u8, 62u8, 63u8, 64u8, 65u8, 66u8, 67u8, 68u8, 69u8, 70u8, 71u8,
            72u8, 73u8, 74u8, 75u8, 76u8, 77u8, 78u8, 79u8, 80u8, 81u8, 82u8, 83u8, 84u8, 85u8,
            86u8, 87u8, 88u8, 89u8, 90u8, 91u8, 92u8, 93u8, 94u8, 95u8, 223u8, 224u8, 225u8, 226u8,
            227u8, 228u8, 229u8, 230u8, 231u8, 232u8, 233u8, 234u8, 235u8, 236u8, 237u8, 238u8,
            239u8, 240u8, 241u8, 242u8, 243u8, 244u8, 245u8, 246u8, 247u8, 248u8, 94u8, 249u8,
            250u8, 251u8, 95u8, 96u8, 97u8, 98u8, 99u8, 100u8, 101u8, 102u8, 103u8, 104u8, 105u8,
            106u8, 107u8, 108u8, 109u8, 110u8, 111u8, 112u8, 113u8, 114u8, 115u8, 116u8, 117u8,
            118u8, 119u8, 120u8, 121u8, 122u8, 123u8, 124u8, 125u8, 126u8, 127u8, 128u8, 129u8,
            130u8, 131u8, 132u8, 133u8, 134u8, 135u8, 136u8, 137u8, 138u8, 139u8, 140u8, 141u8,
            142u8, 143u8, 144u8, 145u8, 146u8, 147u8, 148u8, 149u8, 150u8, 151u8, 152u8, 153u8,
            154u8, 155u8, 156u8, 157u8, 158u8, 159u8, 160u8, 161u8, 162u8, 163u8, 164u8, 165u8,
            166u8, 167u8, 168u8, 169u8, 170u8, 171u8, 172u8, 173u8, 174u8, 175u8, 176u8, 177u8,
            178u8, 179u8, 180u8, 181u8, 182u8, 183u8, 184u8, 185u8, 186u8, 187u8, 188u8, 189u8,
            190u8, 191u8, 192u8, 193u8, 194u8, 195u8, 196u8, 197u8, 198u8, 199u8, 200u8, 201u8,
            202u8, 203u8, 204u8, 205u8, 206u8, 207u8, 208u8, 209u8, 210u8, 211u8, 212u8, 213u8,
            214u8, 215u8, 216u8, 217u8, 218u8, 219u8, 220u8, 221u8, 222u8, 254u8, 255u8,
        ])
    },
    reorder_ranges: unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[]) },
};
static TH: DataStruct = &::icu_collator::provider::CollationReorderingV1 {
    min_high_no_reorder: 1929379840u32,
    reorder_table: unsafe {
        ::zerovec::ZeroVec::from_bytes_unchecked(&[
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
            16u8, 17u8, 18u8, 19u8, 20u8, 21u8, 22u8, 23u8, 24u8, 25u8, 26u8, 27u8, 28u8, 29u8,
            30u8, 31u8, 32u8, 33u8, 34u8, 35u8, 36u8, 37u8, 38u8, 39u8, 40u8, 41u8, 42u8, 43u8,
            44u8, 45u8, 46u8, 47u8, 48u8, 49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8, 57u8,
            58u8, 59u8, 60u8, 61u8, 62u8, 63u8, 64u8, 65u8, 66u8, 67u8, 68u8, 69u8, 70u8, 71u8,
            72u8, 73u8, 74u8, 75u8, 76u8, 77u8, 78u8, 79u8, 80u8, 81u8, 82u8, 83u8, 84u8, 85u8,
            86u8, 87u8, 88u8, 89u8, 90u8, 91u8, 92u8, 93u8, 94u8, 95u8, 96u8, 97u8, 98u8, 99u8,
            100u8, 101u8, 102u8, 103u8, 104u8, 105u8, 106u8, 107u8, 108u8, 109u8, 110u8, 111u8,
            112u8, 113u8, 39u8, 115u8, 116u8, 117u8, 118u8, 119u8, 120u8, 121u8, 122u8, 123u8,
            124u8, 125u8, 126u8, 127u8, 128u8, 129u8, 130u8, 131u8, 132u8, 133u8, 134u8, 135u8,
            136u8, 137u8, 138u8, 139u8, 140u8, 141u8, 142u8, 143u8, 144u8, 145u8, 146u8, 147u8,
            148u8, 149u8, 150u8, 151u8, 152u8, 153u8, 154u8, 155u8, 156u8, 157u8, 158u8, 159u8,
            160u8, 161u8, 162u8, 163u8, 164u8, 165u8, 166u8, 167u8, 168u8, 169u8, 170u8, 171u8,
            172u8, 173u8, 174u8, 175u8, 176u8, 177u8, 178u8, 179u8, 180u8, 181u8, 182u8, 183u8,
            184u8, 185u8, 186u8, 187u8, 188u8, 189u8, 190u8, 191u8, 192u8, 193u8, 194u8, 195u8,
            196u8, 197u8, 198u8, 199u8, 200u8, 201u8, 202u8, 203u8, 204u8, 205u8, 206u8, 207u8,
            208u8, 209u8, 210u8, 211u8, 212u8, 213u8, 214u8, 215u8, 216u8, 217u8, 218u8, 219u8,
            220u8, 221u8, 222u8, 223u8, 224u8, 225u8, 226u8, 227u8, 228u8, 229u8, 230u8, 231u8,
            232u8, 233u8, 234u8, 235u8, 236u8, 237u8, 238u8, 239u8, 240u8, 241u8, 242u8, 243u8,
            244u8, 245u8, 246u8, 247u8, 248u8, 249u8, 250u8, 251u8, 252u8, 253u8, 254u8, 255u8,
        ])
    },
    reorder_ranges: unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[]) },
};
