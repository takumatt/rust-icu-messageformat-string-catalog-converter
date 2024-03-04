use std::collections::HashMap;

pub struct XCStrings {
  pub source_language: String,
  pub strings: Vec<XCString>,
  pub version_string: String,
}

pub struct XCString {
  pub extraction_state: ExtractionState,
  pub localizations: HashMap<String, Localization>,
}

pub enum ExtractionState {
  Manual
}

pub struct Localization {
  pub string_unit: StringUnit,
}

pub struct StringUnit {
  pub localization_state: LocalizationState,
  pub value: String,
}

pub enum LocalizationState {
  Translated
}
