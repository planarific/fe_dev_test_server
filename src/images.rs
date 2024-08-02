use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use crate::Model;
use uuid::Uuid;

pub static IMAGES: LazyLock<Mutex<HashMap<Uuid, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn hash_thumbnail(thumb: String) -> String {
    let id = Uuid::new_v4();
    IMAGES.lock().unwrap().insert(id, thumb);
    id.to_string()
}

pub fn unhash_thumbnail(hash: &Uuid) -> Option<String> {
    let tmp = IMAGES.lock().unwrap().get(hash).map(|s| s.clone());
    tmp
}

pub fn obfuscate(models: &HashMap<u32, Model>) -> HashMap<u32, Model> {
    let mut h = HashMap::new();

    for (k, v) in models.iter() {
        let mut v2: Model = v.clone();
        v2.thumbnail = format!("/v1/thumbs/{}.png", hash_thumbnail(v.thumbnail.clone()));
        h.insert(*k, v2);
    }
    h
}
