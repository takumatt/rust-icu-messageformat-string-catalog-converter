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

// Note: CrowdinString is kept here as it relates to message formats
// TODO: Consider moving to a separate integration module if it grows
#[derive(Clone, Debug)]
pub struct CrowdinString {
    pub identifier: String,
    pub source_string: String,
    pub translation: String,
    pub context: String,
    // pub labels: Option<Vec<String>>,
    pub max_length: Option<u32>,
} 