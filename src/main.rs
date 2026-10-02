use std::error::Error;
pub mod auth;
pub mod db;
pub mod errors;
pub mod middleware;
pub mod models;

use axum::http::{HeaderValue, Method};
use axum::middleware::from_fn;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

use crate::auth::CurrentUser;
use crate::errors::AppError;
use crate::models::url::UrlClicks;
use crate::models::user::User;
use crate::{db::Appstate, models::url::Url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let state = db::Appstate::connect_db().await?;

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ])
        .allow_credentials(true);

    let protected_routes = Router::new()
        .route("/api/urls", get(list_url_table))
        .route("/api/urls/{id}", delete(delete_url))
        .route("/api/url", post(create_short_url))
        .route("/{short_code}", get(redirect_url))
        .route("/api/urls/{id}/analytics", get(get_url_analytics))
        .route_layer(from_fn(middleware::require_auth));

    let app = Router::new()
        .route("/testing", get(testing))
        .route("/db-check", get(db_check))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/users", get(list_users2))
        .route("/urlclicks", get(list_clicks))
        .merge(protected_routes)
        .layer(cors)
        .with_state(state);

    let addr = "0.0.0.0:5000";
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening at: {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn testing() -> (StatusCode, Json<Value>) {
    println!("{:<12} --testing", "HANDLER");

    let res = json!({
        "ok":"Server running"
    });

    (StatusCode::OK, Json(res))
}

async fn db_check(State(state): State<db::Appstate>) -> String {
    println!("{:<12} --dbcheck", "HANDLER");
    match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db)
        .await
    {
        Ok(row) => format!("db says:{}", row),
        Err(e) => format!("quer failed: {:?}", e),
    }
}

#[derive(Deserialize)]
struct CreateUrlRequest {
    url: String,
}

async fn create_short_url(
    State(state): State<db::Appstate>,
    CurrentUser(user_id): CurrentUser,
    Json(payload): Json<CreateUrlRequest>,
) -> Result<Json<models::url::Url>, errors::AppError> {
    println!("{:<12} --create-url", "HANDLER");
    let url = models::url::create_url(&state.db, user_id, &payload.url).await?;

    Ok(Json(url))
}

async fn list_url_table(
    CurrentUser(user_id): CurrentUser,
    State(state): State<Appstate>,
) -> Result<Json<Vec<Url>>, errors::AppError> {
    println!("{:<12} --list-urls", "HANDLER");

    println!("Authenticated User: {}", user_id);

    let urls = models::url::list_urls(&state.db, user_id).await?;

    Ok(Json(urls))
}

async fn redirect_url(
    State(state): State<Appstate>,
    Path(short_code): Path<String>,
) -> Result<Redirect, errors::AppError> {
    let url = models::url::get_url_by_code(&state.db, &short_code).await?;

    println!("url:{:#?}", url);

    models::url::record_click(&state.db, url.id).await?;

    Ok(Redirect::temporary(&url.original_url))
}

async fn delete_url(
    State(state): State<Appstate>,
    Path(id): Path<uuid::Uuid>,
    CurrentUser(user_id): CurrentUser,
) -> Result<StatusCode, errors::AppError> {
    models::url::delete_url(&state.db, id, user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}
#[derive(Serialize)]
struct UserResponse {
    id: Uuid,
    email: String,
}

async fn register(
    State(state): State<Appstate>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, errors::AppError> {
    let user = models::user::create_user(&state.db, &payload.email, &payload.password).await?;
    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
    }))
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}
#[derive(Serialize)]
struct LoginResponse {
    user: UserResponse,
    token: String,
}

async fn login(
    State(state): State<Appstate>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, errors::AppError> {
    let user = models::user::get_user_by_email(&state.db, &payload.email)
        .await
        .map_err(|_| errors::AppError::Unauthorized)?;

    let valid = models::user::verify_password(&payload.password, &user.password_hash)
        .map_err(|_| errors::AppError::Unauthorized)?;

    if !valid {
        return Err(errors::AppError::Unauthorized);
    }

    let token = auth::create_token(user.id).map_err(|_| errors::AppError::Internal)?;

    Ok(Json(LoginResponse {
        user: UserResponse {
            id: user.id,
            email: user.email,
        },
        token,
    }))
}
#[axum::debug_handler]
async fn list_users2(State(state): State<Appstate>) -> Result<Json<Vec<User>>, AppError> {
    let users = models::user::list_users(&state.db).await?;

    Ok(Json(users))
}

#[axum::debug_handler]
async fn list_clicks(State(state): State<Appstate>) -> Result<Json<Vec<UrlClicks>>, AppError> {
    let url_clicks = models::url::get_url_count(&state.db).await?;

    Ok(url_clicks)
}

async fn get_url_analytics(
    State(state): State<Appstate>,
    Path(id): Path<uuid::Uuid>,
    CurrentUser(user_id): CurrentUser,
) -> Result<Json<models::url::UrlAnalytics>, errors::AppError> {
    let analytics = models::url::get_url_analytics(&state.db, id, user_id).await?;

    Ok(Json(analytics))
}
