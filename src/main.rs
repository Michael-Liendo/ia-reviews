use std::env;

use axum::Router;
use dotenv::dotenv;
use sea_orm::Database;

mod controllers;
mod error;
mod repository;
mod routes;
mod services;
mod types;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    println!("->> DATABASE CONNECTED\n");

    let state = types::state::AppState { conn };

    // build our application with a single route
    let app: Router = Router::new()
        .nest("/api", routes::create_routes())
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(server_url).await.unwrap();
    let local_addr = listener.local_addr().unwrap();
    println!("->> LISTENING ON http://{:?}\n", local_addr);

    axum::serve(listener, app)
        .await
        .expect("Failed to initialize the API server");
}
