use crate::xcstrings::{ExtractionState, LocalizationState};
use linked_hash_map;
use serde::Deserialize;
#[derive(Clone, Debug, Deserialize)]
pub struct LocalizableICUMessage {
    pub key: String,
    pub messages: linked_hash_map::LinkedHashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct ConverterOptions {
    pub extraction_state: ExtractionState,
    pub localization_state: LocalizationState,
}

impl ConverterOptions {
    #[inline]
    pub fn default() -> ConverterOptions {
        ConverterOptions {
            extraction_state: ExtractionState::Manual,
            localization_state: LocalizationState::Translated,
        }
    }
}
