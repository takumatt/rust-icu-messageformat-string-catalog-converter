use linked_hash_map::LinkedHashMap;
use serde::Serialize;

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
    Manual,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Localization {
    pub string_unit: StringUnit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<LinkedHashMap<String, Substitution>>
}

#[derive(Clone, Debug, Serialize)]
pub struct StringUnit {
    #[serde(rename = "state")]
    pub localization_state: LocalizationState,
    pub value: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalizationState {
    Translated,
    NeedsReview,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Substitution {
    pub arg_num: usize,
    pub format_specifier: String,
    pub variations: VariationType
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VariationType {
    Plural(LinkedHashMap<String, VariationValue>)
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VariationValue {
    pub string_unit: StringUnit
}
