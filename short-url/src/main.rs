mod config;
mod handler;
mod model;
mod view;

use crate::config::Config;
use dotenvy::dotenv;
use handler::AppState;
use util::Dao;

use axum::{routing::get, Extension, Router};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "short_url=debug");
    }

    tracing_subscriber::fmt::init();

    dotenv().ok();

    let cfg = Config::from_env("short_url").expect("初始化配置失败");

    let dao = Dao::new(cfg.0).await.expect("Dao required");

    let app_state = AppState {
        dao,
        short_url: cfg.1.clone(),
    };

    let app = Router::new()
        .route("/", get(handler::index).post(handler::index_action))
        .route("/rank", get(handler::rank))
        .route("/msg", get(handler::msg))
        .route("/:id", get(handler::goto_url))
        .nest("/static", axum_static::static_router("static"))
        .layer(Extension(app_state));

    tracing::info!("服务器监听于：{}", &cfg.1.ip_addr);

    axum::Server::bind(&cfg.1.ip_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
