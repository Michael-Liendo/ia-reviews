use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let local_addr = listener.local_addr().unwrap();
    println!("->> LISTENING ON http://{:?}\n", local_addr);

    axum::serve(listener, app)
        .await
        .expect("Failed to initialize the API server");
}
