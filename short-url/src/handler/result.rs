use axum::{
    http::{HeaderMap, StatusCode},
    response::Html,
};

pub type Result<T> = std::result::Result<T, util::ApiError>;
pub type HandlerResult<T> = self::Result<T>;
pub type RedirectResponse = (StatusCode, HeaderMap, ());
pub type HandlerRedirectResult = self::HandlerResult<RedirectResponse>;
pub type HtmlResponse = Html<String>;
pub type HandlerHtmlResult = HandlerResult<HtmlResponse>;
