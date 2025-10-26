use axum::{Extension, Json, http::StatusCode};
use crate::{
    auth::{create_jwt, hash_password, verify_password},
    model::{LoginRequest, LoginResponse, RegisterRequest, UserPublic},
    user_service::UserService,
};

pub async fn register(
    service: Extension<UserService>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Hash the password
    let password_hash = hash_password(&req.password)
        .map_err(|e| {
            eprintln!("Password hashing error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Create the user
    let user = service
        .create_user_with_auth(req.email, password_hash, req.name, req.occupation)
        .await
        .map_err(|e| {
            eprintln!("User creation error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Generate JWT token
    let token = create_jwt(user.id, &user.email)
        .map_err(|e| {
            eprintln!("JWT creation error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(LoginResponse {
        token,
        user: UserPublic {
            id: user.id,
            email: user.email,
            name: user.name,
            occupation: user.occupation,
        },
    }))
}

pub async fn login(
    service: Extension<UserService>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Get user by email
    let user = service
        .get_user_by_email(&req.email)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Verify password
    let is_valid = verify_password(&req.password, &user.password_hash)
        .map_err(|e| {
            eprintln!("Password verification error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Generate JWT token
    let token = create_jwt(user.id, &user.email)
        .map_err(|e| {
            eprintln!("JWT creation error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(LoginResponse {
        token,
        user: UserPublic {
            id: user.id,
            email: user.email,
            name: user.name,
            occupation: user.occupation,
        },
    }))
}
