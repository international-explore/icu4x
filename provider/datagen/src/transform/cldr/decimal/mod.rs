// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use crate::transform::reader::{get_langid_subdirectories, get_langid_subdirectory, open_reader};
use crate::SourceData;
use icu_decimal::provider::*;
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use litemap::LiteMap;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::sync::RwLock;
use tinystr::TinyStr8;

mod decimal_pattern;

/// A data provider reading from CLDR JSON plural rule files.
#[derive(Debug)]
pub struct NumbersProvider {
    source: SourceData,
    cldr_numbering_systems_data:
        RwLock<Option<LiteMap<TinyStr8, cldr_serde::numbering_systems::NumberingSystem>>>,
}

impl From<&SourceData> for NumbersProvider {
    fn from(source: &SourceData) -> Self {
        NumbersProvider {
            source: source.clone(),
            cldr_numbering_systems_data: RwLock::new(None),
        }
    }
}

impl NumbersProvider {
    /// Returns the digits for the given numbering system name.
    fn get_digits_for_numbering_system(&self, nsname: TinyStr8) -> Option<[char; 10]> {
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        match self
            .cldr_numbering_systems_data
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .get(&nsname)
        {
            Some(ns) => match ns.digits.as_ref() {
                Some(digits_str) => {
                    let mut chars = digits_str.chars();
                    Some([
                        chars.next()?,
                        chars.next()?,
                        chars.next()?,
                        chars.next()?,
                        chars.next()?,
                        chars.next()?,
                        chars.next()?,
                        chars.next()?,
                        chars.next()?,
                        chars.next()?,
                    ])
                }
                None => None,
            },
            None => None,
        }
    }
}

impl ResourceProvider<DecimalSymbolsV1Marker> for NumbersProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<DecimalSymbolsV1Marker>, DataError> {
        let langid = req.options.get_langid();

        let resource: cldr_serde::numbers::Resource = {
            let path = get_langid_subdirectory(
                &self.source.get_cldr_paths()?.cldr_numbers().join("main"),
                &langid,
            )?
            .ok_or_else(|| DataErrorKind::MissingLocale.into_error())?
            .join("numbers.json");
            serde_json::from_reader(open_reader(&path)?)
                .map_err(|e| DataError::from(e).with_path_context(&path))?
        };

        #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        let numbers = &resource
            .main
            .0
            .get(&langid)
            .expect("CLDR file contains the expected language")
            .numbers;
        let nsname = numbers.default_numbering_system;

        let mut result = DecimalSymbolsV1::try_from(numbers).map_err(|s| {
            DataError::custom("Could not create decimal symbols").with_display_context(&s)
        })?;

        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        if self.cldr_numbering_systems_data.read().unwrap().is_none() {
            let path = self
                .source
                .get_cldr_paths()?
                .cldr_core()
                .join("supplemental")
                .join("numberingSystems.json");
            let resource: cldr_serde::numbering_systems::Resource =
                serde_json::from_reader(open_reader(&path)?)
                    .map_err(|e| DataError::from(e).with_path_context(&path))?;
            let _ = self
                .cldr_numbering_systems_data
                .write()
                .unwrap()
                .get_or_insert(resource.supplemental.numbering_systems);
        }

        result.digits = self
            .get_digits_for_numbering_system(nsname)
            .ok_or_else(|| {
                DataError::custom("Could not process numbering system")
                    .with_display_context(&nsname)
            })?;

        let metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}

icu_provider::impl_dyn_provider!(
    NumbersProvider,
    [DecimalSymbolsV1Marker,],
    SERDE_SE,
    CRABBAKE,
    ITERABLE_SERDE_SE,
    ITERABLE_CRABBAKE,
    DATA_CONVERTER
);

impl IterableResourceProvider<DecimalSymbolsV1Marker> for NumbersProvider {
    fn supported_options(
        &self,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        Ok(Box::new(
            get_langid_subdirectories(&self.source.get_cldr_paths()?.cldr_numbers().join("main"))?
                .map(Into::<ResourceOptions>::into),
        ))
    }
}

impl TryFrom<&cldr_serde::numbers::Numbers> for DecimalSymbolsV1<'static> {
    type Error = Cow<'static, str>;

    fn try_from(other: &cldr_serde::numbers::Numbers) -> Result<Self, Self::Error> {
        // TODO(#510): Select from non-default numbering systems
        let symbols = other
            .numsys_data
            .symbols
            .get(&other.default_numbering_system)
            .ok_or("Could not find symbols for default numbering system")?;
        let formats = other
            .numsys_data
            .formats
            .get(&other.default_numbering_system)
            .ok_or("Could not find formats for default numbering system")?;
        let parsed_pattern: decimal_pattern::DecimalPattern = formats
            .standard
            .parse()
            .map_err(|s: decimal_pattern::Error| s.to_string())?;

        Ok(Self {
            minus_sign_affixes: parsed_pattern.localize_sign(&symbols.minus_sign),
            plus_sign_affixes: parsed_pattern.localize_sign(&symbols.plus_sign),
            decimal_separator: Cow::Owned(symbols.decimal.clone()),
            grouping_separator: Cow::Owned(symbols.group.clone()),
            grouping_sizes: GroupingSizesV1 {
                primary: parsed_pattern.positive.primary_grouping,
                secondary: parsed_pattern.positive.secondary_grouping,
                min_grouping: other.minimum_grouping_digits,
            },
            digits: Default::default(), // to be filled in
        })
    }
}

#[test]
fn test_basic() {
    use icu_locid::locale;

    let provider = NumbersProvider::from(&SourceData::for_test());

    let ar_decimal: DataPayload<DecimalSymbolsV1Marker> = provider
        .load_resource(&DataRequest {
            options: locale!("ar-EG").into(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!(ar_decimal.get().decimal_separator, "٫");
    assert_eq!(ar_decimal.get().digits[0], '٠');
}
