use axum::Json;
use chrono::{DateTime, Utc};
use rand::RngExt;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Url {
    pub id: Uuid,
    pub short_code: String,
    pub original_url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct UrlClicks {
    pub id: Uuid,
    pub url_id: Uuid,
    pub clicked_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UrlAnalytics {
    pub url_id: Uuid,
    pub total_clicks: i64,
}

pub fn generate_short_code() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

    let mut rng = rand::rng();

    (0..6)
        .map(|_x| {
            let index = rng.random_range(0..CHARSET.len());
            CHARSET[index] as char
        })
        .collect()
}

pub async fn create_url(
    pool: &PgPool,
    user_id: Uuid,
    original_url: &str,
) -> Result<Url, sqlx::Error> {
    let id = Uuid::new_v4();
    let short_code = generate_short_code();

    let url = sqlx::query_as::<_, Url>(
        r#"
    INSERT INTO urls (id,short_code,original_url,user_id)
    Values($1,$2,$3,$4)
    RETURNING *
    "#,
    )
    .bind(id)
    .bind(&short_code)
    .bind(original_url)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(url)
}

pub async fn get_url_by_code(pool: &PgPool, short_code: &str) -> Result<Url, sqlx::Error> {
    let url = sqlx::query_as::<_, Url>(r#"SELECT * FROM urls WHERE short_code = $1"#)
        .bind(short_code)
        .fetch_one(pool)
        .await?;
    Ok(url)
}

pub async fn list_urls(pool: &PgPool, user_id: Uuid) -> Result<Vec<Url>, sqlx::Error> {
    let urls = sqlx::query_as::<_, Url>(
        r#"
        SELECT * FROM urls WHERE user_id =$1 ORDER BY created_at DESC
    "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(urls)
}

pub async fn delete_url(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
    let result = sqlx::query(
        r#"
    DELETE FROM urls
    WHERE id = $1 AND user_id = $2
    "#,
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}

pub async fn record_click(pool: &PgPool, url_id: Uuid) -> Result<(), sqlx::Error> {
    let click_id = Uuid::new_v4();

    sqlx::query(
        r#"
    INSERT INTO url_clicks (id,url_id)
    VALUES($1,$2)
    "#,
    )
    .bind(click_id)
    .bind(url_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_url_count(pool: &PgPool) -> Result<Json<Vec<UrlClicks>>, sqlx::Error> {
    let urls = sqlx::query_as::<_, UrlClicks>(
        r#"
    SELECT * FROM url_clicks ORDER BY clicked_at DESC
    "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(Json(urls))
}

pub async fn get_url_analytics(
    pool: &PgPool,
    url_id: Uuid,
    user_id: Uuid,
) -> Result<UrlAnalytics, sqlx::Error> {
    let analytics = sqlx::query_as::<_, UrlAnalytics>(
        r#"
    SELECT
        urls.id as url_id,
        COUNT(url_clicks.id) AS total_clicks
    FROM urls
    LEFT JOIN url_clicks
        ON url_clicks.url_id = urls.id
    WHERE urls.id =$1
    AND urls.user_id = $2
    GROUP BY urls.id
    "#,
    )
    .bind(url_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(analytics)
}
