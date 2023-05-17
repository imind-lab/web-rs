mod short_url;

pub use short_url::ShortUrl;
use util::Config as Dao;

pub struct Config;

impl Config {
    pub fn from_env(prefix: &str) -> Result<(Dao, ShortUrl), config::ConfigError> {
        let dao = Dao::from_env(prefix)?;
        let short_url = ShortUrl::from_env()?;
        Ok((dao, short_url))
    }
}
