use linked_hash_map;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LocalizableICUMessage {
   pub key: String,
   pub messages: linked_hash_map::LinkedHashMap<String, String>,
}
