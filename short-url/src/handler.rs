mod core;
pub mod result;

use askama::Template;
use result::{HandlerHtmlResult, HandlerRedirectResult};

use axum::{
    extract::{Extension, Form, Path, Query},
    http::{HeaderMap, StatusCode},
    response::Html,
};

use crate::config;
use crate::model::{self, CreateUrl};
use crate::view::{IndexTemplate, MsgTemplate, RankTemplate};
use result::*;
use util::ApiError;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct AppState {
    pub dao: util::Dao,
    pub short_url: config::ShortUrl,
}

fn redirect_with_msg(url: &str, args: Option<&MsgArgs>) -> RedirectResponse {
    let url = match args {
        Some(args) => {
            format!("{}?{}", url, args.to_string())
        }
        None => url.to_string(),
    };
    let mut header = HeaderMap::new();
    header.insert(axum::http::header::LOCATION, url.as_str().parse().unwrap());
    (StatusCode::FOUND, header, ())
}

fn redirect(url: &str) -> RedirectResponse {
    redirect_with_msg(url, None)
}

fn render<T: Template>(t: T) -> HandlerHtmlResult {
    let html = t.render().map_err(ApiError::TemplateError)?;
    Ok(Html(html))
}

fn log_error(handler_name: String) -> Box<dyn Fn(ApiError) -> ApiError> {
    Box::new(move |err| {
        tracing::error!("{}: {:?}", handler_name, err);
        err
    })
}

pub async fn index_action(
    Extension(state): Extension<AppState>,
    Form(cu): Form<CreateUrl>,
) -> HandlerRedirectResult {
    let id = core::short_url(&cu.url);

    if state.short_url.in_reserved_words(&id) {
        return Err(ApiError::ReservedWord(id));
    };
    let handler_name = "index_action";

    let dao = &state.dao;

    let result = model::create(dao, cu, id)
        .await
        .map_err(log_error(handler_name.to_string()))?;

    let redirect_url = format!("/?id={}", result.id);
    Ok(redirect(&redirect_url))
}

#[derive(Deserialize)]
pub struct IndexArgs {
    pub id: Option<String>,
}

pub async fn index(
    Extension(state): Extension<AppState>,
    Query(args): Query<IndexArgs>,
) -> HandlerHtmlResult {
    let handler_name = "index";
    let tmpl = IndexTemplate {
        id: args.id,
        short_url_domain: state.short_url.domain,
    };
    render(tmpl).map_err(log_error(handler_name.to_string()))
}

pub async fn goto_url(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> HandlerRedirectResult {
    let handler_name = "goto_url";
    let dao = &state.dao;
    let result = model::goto_url(dao, id)
        .await
        .map_err(log_error(handler_name.to_string()))?;

    Ok(redirect(result.url.as_str()))
}

pub async fn rank(Extension(state): Extension<AppState>) -> HandlerHtmlResult {
    let handler_name = "rank";
    let dao = &state.dao;
    let result = model::rank(dao)
        .await
        .map_err(log_error(handler_name.to_string()))?;

    let tmpl = RankTemplate {
        urls: result,
        short_url_domain: state.short_url.domain.clone(),
    };
    render(tmpl).map_err(log_error(handler_name.to_string()))
}

pub async fn msg(Query(args): Query<MsgArgs>) -> HandlerHtmlResult {
    let handler_name = "err";
    let tmpl: MsgTemplate = args.into();

    render(tmpl).map_err(log_error(handler_name.to_string()))
}

#[derive(Deserialize)]
pub struct MsgArgs {
    pub ok: Option<String>,
    pub err: Option<String>,
    pub target: Option<String>,
}

impl ToString for MsgArgs {
    fn to_string(&self) -> String {
        let mut r: Vec<String> = vec![];
        if let Some(target) = self.target.clone() {
            r.push(format!("target={}", target));
        }
        if let Some(msg) = self.ok.clone() {
            r.push(format!("ok={}", msg));
        }
        if let Some(msg) = self.err.clone() {
            r.push(format!("err={}", msg));
        }
        r.join("&")
    }
}

impl From<MsgArgs> for MsgTemplate {
    fn from(val: MsgArgs) -> Self {
        let mut tmpl = MsgTemplate {
            target_url: val.target.clone(),
            ..Default::default()
        };
        match val {
            MsgArgs { ok: Some(msg), .. } => {
                tmpl.is_ok = true;
                tmpl.msg = msg;
            }
            MsgArgs { err: Some(msg), .. } => {
                tmpl.is_ok = false;
                tmpl.msg = msg;
            }
            _ => {}
        }
        tmpl
    }
}
