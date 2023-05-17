use askama::Template;

use crate::model::Url;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub id: Option<String>,
    pub short_url_domain: String,
}

impl IndexTemplate {
    pub fn id(&self) -> String {
        self.id.clone().unwrap_or("".to_string())
    }
}

#[derive(Template)]
#[template(path = "rank.html")]
pub struct RankTemplate {
    pub urls: Vec<Url>,
    pub short_url_domain: String,
}

#[derive(Template)]
#[template(path = "msg.html")]
pub struct MsgTemplate {
    pub is_ok: bool,
    pub msg: String,
    pub target_url: Option<String>,
}

impl MsgTemplate {
    fn new(is_ok: bool, msg: String, target_url: Option<String>) -> Self {
        Self {
            is_ok,
            msg,
            target_url,
        }
    }
    #[allow(unused)]
    pub fn err(msg: String) -> Self {
        Self::new(false, msg, None)
    }
    pub fn target_url(&self) -> String {
        match self.target_url.clone() {
            Some(target_url) => target_url,
            None => "javascript:history.back(-1)".to_string(),
        }
    }
}

impl Default for MsgTemplate {
    fn default() -> Self {
        Self::new(false, String::from(""), None)
    }
}
