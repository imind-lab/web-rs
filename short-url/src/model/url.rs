use futures::TryStreamExt;
use util::{Dao, DataOperator};

use crate::{
    handler::result::Result,
    model::{CreateUrl, Url, UrlID, UrlTarget},
};

pub async fn create(dao: &Dao, cu: CreateUrl, id: String) -> Result<UrlID> {
    let result = sqlx::query_as::<_, UrlID>("SELECT id FROM url WHERE id=?")
        .bind(&id)
        .fetch_one(dao.get_database_pool())
        .await;

    if let Ok(url_id) = result {
        return Ok(url_id);
    };

    sqlx::query("INSERT INTO url(id, url, email) VALUES (?, ?, ?)")
        .bind(&id)
        .bind(cu.url)
        .bind(cu.email)
        .execute(dao.get_database_pool())
        .await?;

    Ok(UrlID { id })
}

pub async fn goto_url(dao: &Dao, id: String) -> Result<UrlTarget> {
    sqlx::query("UPDATE url SET visit=visit+1 WHERE id=?")
        .bind(&id)
        .execute(dao.get_database_pool())
        .await?;

    let url_target = sqlx::query_as::<_, UrlTarget>("SELECT url FROM url WHERE id=?")
        .bind(&id)
        .fetch_one(dao.get_database_pool())
        .await?;

    Ok(url_target)
}

pub async fn rank(dao: &Dao) -> Result<Vec<Url>> {
    let mut stream =
        sqlx::query_as::<_, Url>("SELECT * FROM url WHERE is_del=? ORDER BY visit DESC LIMIT 100")
            .bind(0)
            .fetch(dao.get_database_pool());

    let mut urls = Vec::new();
    while let Some(url) = stream.try_next().await? {
        urls.push(url);
    }

    Ok(urls)
}
