use crate::xcstrings::{ExtractionState, LocalizationState};

#[derive(Clone, Debug)]
pub struct ConverterOptions {
    #[allow(dead_code)]
    pub extraction_state: ExtractionState,
    pub localization_state: LocalizationState,
    pub split_select_elements: bool,
}

impl ConverterOptions {
    #[inline]
    pub fn default() -> ConverterOptions {
        ConverterOptions {
            extraction_state: ExtractionState::Manual,
            localization_state: LocalizationState::Translated,
            split_select_elements: true,
        }
    }
} 