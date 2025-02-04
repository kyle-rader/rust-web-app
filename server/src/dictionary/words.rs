use std::collections::HashSet;

const WORDS_4_RAW: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/words", "/4.txt"));
const WORDS_BANNED_RAW: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/words", "/banned.txt"));

pub struct Words {
    len_4: HashSet<String>,
    banned: HashSet<String>,
}

impl Words {
    fn new() -> Self {
        Self {
            len_4: raw_to_hash_set(WORDS_4_RAW),
            banned: raw_to_hash_set(WORDS_BANNED_RAW),
        }
    }
}

fn raw_to_hash_set(raw: &str) -> HashSet<String> {
    raw.lines().map(|l| l.to_owned()).collect()
}

pub fn words_4() -> Vec<String> {
    WORDS_4_RAW.lines().map(|l| l.to_owned()).collect()
}
