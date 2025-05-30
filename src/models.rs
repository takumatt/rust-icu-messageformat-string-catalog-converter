use crate::xcstrings::{ExtractionState, LocalizationState};
use linked_hash_map;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LocalizableICUString {
    pub key: String,
    pub messages: linked_hash_map::LinkedHashMap<String, LocalizableICUMessageValue>,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct LocalizableICUMessageValue {
    pub value: String,
    #[serde(default = "default_localization_state")]
    pub state: String,
}

fn default_localization_state() -> String {
    "translated".to_string()
}

impl From<LocalizableICUString> for LocalizableICUMessage {
    fn from(string: LocalizableICUString) -> Self {
        LocalizableICUMessage {
            key: string.key,
            messages: string.messages,
            comment: string.comment,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct LocalizableICUStrings {
    pub strings: Vec<LocalizableICUString>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LocalizableICUMessage {
    pub key: String,
    pub messages: linked_hash_map::LinkedHashMap<String, LocalizableICUMessageValue>,
    pub comment: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ConverterOptions {
    pub extraction_state: ExtractionState,
    pub localization_state: LocalizationState,
    pub warn_on_select: bool,
    pub error_on_select: bool,
    pub split_select_elements: bool,
}

impl ConverterOptions {
    #[inline]
    pub fn default() -> ConverterOptions {
        ConverterOptions {
            extraction_state: ExtractionState::Manual,
            localization_state: LocalizationState::Translated,
            warn_on_select: false,
            error_on_select: false,
            split_select_elements: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CrowdinString {
    pub identifier: String,
    pub source_string: String,
    pub translation: String,
    pub context: String,
    // pub labels: Option<Vec<String>>,
    pub max_length: Option<u32>,
}