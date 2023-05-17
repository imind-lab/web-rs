use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct ServiceConfig {
    pub reserved_words: String,
    pub domain: String,
    pub addr: String,
}

impl ServiceConfig {
    pub fn reserved_words(&self) -> Vec<&str> {
        self.reserved_words.split(',').collect()
    }

    pub fn in_reserved_words(&self, word: &str) -> bool {
        for w in self.reserved_words() {
            if w == word {
                return true;
            }
        }
        false
    }
}
