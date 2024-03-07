use std::collections::HashMap;
use linked_hash_map::LinkedHashMap;
use serde::{Serialize};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct XCStrings {
  pub source_language: String,
  pub strings: LinkedHashMap<String, XCString>,
  pub version: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct XCString {
  pub extraction_state: ExtractionState,
  pub localizations: LinkedHashMap<String, Localization>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtractionState {
  Manual
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Localization {
  pub string_unit: StringUnit,
}

#[derive(Clone, Debug, Serialize)]
pub struct StringUnit {
  #[serde(rename="state")]
  pub localization_state: LocalizationState,
  pub value: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LocalizationState {
  Translated
}
