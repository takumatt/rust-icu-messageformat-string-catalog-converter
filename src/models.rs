use linked_hash_map;
use serde::Deserialize;
use crate::xcstrings::{ExtractionState, LocalizationState};
#[derive(Clone, Debug, Deserialize)]
pub struct LocalizableICUMessage {
    pub key: String,
    pub messages: linked_hash_map::LinkedHashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct ConverterOptions {
    pub extractionState: ExtractionState,
    pub localizationState: LocalizationState,
}

impl ConverterOptions {
    #[inline]
    pub fn default() -> ConverterOptions {
        ConverterOptions {
            extractionState: ExtractionState::Manual,
            localizationState: LocalizationState::Translated,
        }
    }
}