use std::collections::HashMap;
use serde::{Serialize};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct XCStrings {
  pub source_language: String,
  pub strings: Vec<XCString>,
  pub version: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct XCString {
  pub extraction_state: ExtractionState,
  #[serde(flatten)]
  pub localizations: HashMap<String, Localization>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtractionState {
  Manual
}

#[derive(Clone, Debug, Serialize)]
pub struct Localization {
  pub string_unit: StringUnit,
}

#[derive(Clone, Debug, Serialize)]
pub struct StringUnit {
  pub localization_state: LocalizationState,
  pub value: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LocalizationState {
  Translated
}
