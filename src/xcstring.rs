use std::collections::HashMap;
use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct XCStrings {
  pub source_language: String,
  pub strings: Vec<XCString>,
  pub version_string: String,
}

#[derive(Debug, Serialize)]
pub struct XCString {
  pub extraction_state: ExtractionState,
  pub localizations: HashMap<String, Localization>,
}

#[derive(Debug, Serialize)]
pub enum ExtractionState {
  Manual
}

#[derive(Debug, Serialize)]
pub struct Localization {
  pub string_unit: StringUnit,
}

#[derive(Debug, Serialize)]
pub struct StringUnit {
  pub localization_state: LocalizationState,
  pub value: String,
}

#[derive(Debug, Serialize)]
pub enum LocalizationState {
  Translated
}
