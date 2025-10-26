use axum::{
    Extension, Router,
    routing::{delete, get, post, put},
};

use crate::{
    auth_controller::{login, register},
    controller::{delete_user, get_user_by_id, list_users, update_user},
    middleware::auth_middleware,
    user_service::UserService,
};

mod auth;
mod auth_controller;
mod controller;
mod middleware;
mod model;
mod user_service;

#[tokio::main]
async fn main() {
    println!("Starting service..!");
    let service = UserService::new().await.unwrap();

    // Public routes
    let public_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/users", get(list_users))
        .route("/user/{id}", get(get_user_by_id));

    // Protected routes (require authentication)
    let protected_routes = Router::new()
        .route("/user/{id}", put(update_user))
        .route("/user/{id}", delete(delete_user))
        .route_layer(axum::middleware::from_fn(auth_middleware));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(Extension(service));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening..!");
    axum::serve(listener, app).await.unwrap();
}
