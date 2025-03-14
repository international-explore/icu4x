// @generated
use icu_provider::prelude::*;
impl ResourceProvider<::icu_datetime::provider::week_data::WeekDataV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<::icu_datetime::provider::week_data::WeekDataV1Marker>, DataError>
    {
        static VALUES: &[(&str, DataStruct)] = &[
            ("und", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-AD", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-AE", UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG),
            ("und-AF", UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG),
            ("und-AG", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-AI", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-AL", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-AM", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-AN", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-AR", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-AS", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-AT", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-AU", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-AX", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-AZ", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-BA", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-BD", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-BE", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-BG", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-BH", UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG),
            ("und-BM", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-BN", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-BR", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-BS", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-BT", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-BW", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-BY", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-BZ", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-CA", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-CH", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-CL", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-CM", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-CN", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-CO", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-CR", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-CY", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-CZ", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-DE", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-DJ", UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG),
            ("und-DK", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-DM", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-DO", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-DZ", UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG),
            ("und-EC", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-EE", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-EG", UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG),
            ("und-ES", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-ET", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-FI", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-FJ", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-FO", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-FR", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-GB", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-GE", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-GF", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-GG", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-GI", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-GP", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-GR", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-GT", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-GU", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-HK", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-HN", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-HR", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-HU", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-ID", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-IE", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-IL", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-IM", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-IN", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-IQ", UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG),
            ("und-IR", UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG),
            ("und-IS", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-IT", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-JE", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-JM", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-JO", UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG),
            ("und-JP", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-KE", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-KG", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-KH", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-KR", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-KW", UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG),
            ("und-KZ", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-LA", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-LB", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-LI", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-LK", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-LT", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-LU", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-LV", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-LY", UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG),
            ("und-MC", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-MD", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-ME", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-MH", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-MK", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-MM", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-MN", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-MO", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-MQ", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-MT", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-MV", UND_MV),
            ("und-MX", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-MY", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-MZ", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-NI", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-NL", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-NO", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-NP", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-NZ", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-OM", UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG),
            ("und-PA", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-PE", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-PH", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-PK", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-PL", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-PR", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-PT", UND_PT),
            ("und-PY", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-QA", UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG),
            ("und-RE", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-RO", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-RS", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-RU", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-SA", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-SD", UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG),
            ("und-SE", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-SG", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-SI", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-SJ", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-SK", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-SM", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-SV", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-SY", UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG),
            ("und-TH", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-TJ", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-TM", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-TR", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-TT", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-TW", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-UA", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-UM", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-US", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-UY", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-UZ", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-VA", UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG),
            ("und-VE", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-VI", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-VN", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-WS", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-XK", UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU),
            ("und-YE", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-ZA", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
            ("und-ZW", UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT),
        ];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions.with_req(
                    <::icu_datetime::provider::week_data::WeekDataV1Marker>::KEY,
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
    &'static <::icu_datetime::provider::week_data::WeekDataV1Marker as DataMarker>::Yokeable;
static UND_AD_UND_AN_UND_AT_UND_AX_UND_BE_UND_BG: DataStruct =
    &::icu_datetime::provider::week_data::WeekDataV1(
        ::icu_calendar::arithmetic::week_of::CalendarInfo {
            first_weekday: ::icu_calendar::types::IsoWeekday::Monday,
            min_week_days: 4u8,
        },
    );
static UND_AE_UND_AF_UND_BH_UND_DJ_UND_DZ_UND_EG: DataStruct =
    &::icu_datetime::provider::week_data::WeekDataV1(
        ::icu_calendar::arithmetic::week_of::CalendarInfo {
            first_weekday: ::icu_calendar::types::IsoWeekday::Saturday,
            min_week_days: 1u8,
        },
    );
static UND_AG_UND_AS_UND_BD_UND_BR_UND_BS_UND_BT: DataStruct =
    &::icu_datetime::provider::week_data::WeekDataV1(
        ::icu_calendar::arithmetic::week_of::CalendarInfo {
            first_weekday: ::icu_calendar::types::IsoWeekday::Sunday,
            min_week_days: 1u8,
        },
    );
static UND_MV: DataStruct = &::icu_datetime::provider::week_data::WeekDataV1(
    ::icu_calendar::arithmetic::week_of::CalendarInfo {
        first_weekday: ::icu_calendar::types::IsoWeekday::Friday,
        min_week_days: 1u8,
    },
);
static UND_PT: DataStruct = &::icu_datetime::provider::week_data::WeekDataV1(
    ::icu_calendar::arithmetic::week_of::CalendarInfo {
        first_weekday: ::icu_calendar::types::IsoWeekday::Sunday,
        min_week_days: 4u8,
    },
);
static UND_UND_AI_UND_AL_UND_AM_UND_AR_UND_AU: DataStruct =
    &::icu_datetime::provider::week_data::WeekDataV1(
        ::icu_calendar::arithmetic::week_of::CalendarInfo {
            first_weekday: ::icu_calendar::types::IsoWeekday::Monday,
            min_week_days: 1u8,
        },
    );
