use axum::{
    body::Body,
    http::{Request, header},
    middleware::Next,
    response::Response,
};

use crate::{auth::CurrentUser, errors::AppError, *};

pub async fn require_auth(
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, errors::AppError> {
    // Get Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(AppError::Unauthorized)?;

    println!("AUTH HEADER: {:?}", auth_header);

    // Convert header value to string
    let auth_header = auth_header.to_str().map_err(|_| AppError::Unauthorized)?;

    println!("AUTH STRING: {:?}", auth_header);

    // Make sure it uses the Bearer scheme
    let token = auth_header
        .strip_prefix("Bearer")
        .ok_or(AppError::Unauthorized)?
        .trim();

    println!("TOKEN: {}", token);
    println!("TOKEN DEBUG: {:?}", token);

    //verify JWT
    let claims = auth::verify_token(token).map_err(|e| {
        println!("JWT ERROR: {:?}", e);
        AppError::Unauthorized
    })?;

    println!("CLAIMS: {:?}", claims);

    //convert sub fromstring -> UUID
    let user_id = claims.sub.parse().map_err(|e| {
        println!("UUID ERROR {:?}", e);
        AppError::Unauthorized
    })?;

    request.extensions_mut().insert(CurrentUser(user_id));

    Ok(next.run(request).await)
}
