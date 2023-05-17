use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct ShortUrl {
    pub reserved_words: String,
    pub domain: String,
    pub ip_addr: String,
}

impl ShortUrl {
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

impl ShortUrl {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(
                config::Environment::default()
                    .prefix("xurl")
                    .try_parsing(true)
                    .separator("."),
            )
            .build()?;
        cfg.try_deserialize()
    }
}
