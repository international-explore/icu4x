// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::LanguageIdentifier;
use icu_provider::DataError;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Helper function to open a file and return failures as a crate error.
pub fn open_reader(path: &Path) -> Result<BufReader<File>, DataError> {
    log::trace!("Reading: {:?}", path);
    File::open(&path)
        .map(BufReader::new)
        .map_err(|e| DataError::from(e).with_path_context(path))
}

/// Read the contents of the file at `path` and return it as a `String`.
pub fn read_path_to_string(path: &Path) -> Result<String, DataError> {
    let mut reader = open_reader(path)?;
    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .map_err(|e| DataError::from(e).with_path_context(path))?;
    Ok(buffer)
}

/// Helper function which returns a sorted list of the contents of a directory.
pub fn get_dir_contents(root: &Path) -> Result<Vec<PathBuf>, DataError> {
    let mut result = vec![];
    for entry in fs::read_dir(root).map_err(|e| DataError::from(e).with_path_context(root))? {
        let path = entry
            .map_err(|e| DataError::from(e).with_path_context(&root))?
            .path();
        result.push(path);
    }
    result.sort();
    Ok(result)
}

fn get_langid_subdirectories_internal(
    root: &Path,
) -> Result<impl Iterator<Item = (LanguageIdentifier, PathBuf)>, DataError> {
    let mut result = vec![];
    for entry in fs::read_dir(root).map_err(|e| DataError::from(e).with_path_context(&root))? {
        let entry = entry.map_err(|e| DataError::from(e).with_path_context(&root))?;
        let path = entry.path();
        result.push(path);
    }
    Ok(result.into_iter().map(|path| {
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        (
            LanguageIdentifier::from_str(&path.file_name().unwrap().to_string_lossy()).unwrap(),
            path,
        )
    }))
}

/// Helper function which returns an unsorted list of langids for which subdirectories exist.
pub fn get_langid_subdirectories(
    root: &Path,
) -> Result<impl Iterator<Item = LanguageIdentifier>, DataError> {
    get_langid_subdirectories_internal(root).map(|iter| iter.map(|(l, _)| l))
}

/// Helper function which returns the subdirectory for the selected language, if it exists.
pub fn get_langid_subdirectory(
    root: &Path,
    langid: &LanguageIdentifier,
) -> Result<Option<PathBuf>, DataError> {
    get_langid_subdirectories_internal(root).map(|mut iter| {
        iter.find(|(langid2, _)| langid2 == langid)
            .map(|(_, path)| path)
    })
}
