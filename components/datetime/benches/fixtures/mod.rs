// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod structs;

use icu_datetime::options::DateTimeFormatOptions;
use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
pub fn get_fixture(name: &str) -> std::io::Result<structs::Fixture> {
    let file = File::open(format!("./benches/fixtures/tests/{}.json", name))?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

#[allow(dead_code)]
pub fn get_patterns_fixture() -> std::io::Result<structs::PatternsFixture> {
    let file = File::open("./benches/fixtures/tests/patterns.json")?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

#[allow(dead_code)]
pub fn get_options(input: &structs::TestOptions) -> DateTimeFormatOptions {
    match input {
        structs::TestOptions::Length(bag) => (*bag).into(),
        structs::TestOptions::Components(bag) => (*bag).into(),
    }
}
