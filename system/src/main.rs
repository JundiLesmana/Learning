use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;
use learning_notes::pages;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(pages::home::handler))
        .route("/about", get(pages::about::handler))
        .route("/mylearn", get(pages::mylearn::handler))
        .route("/Insert-Sorting", get(pages::insert_sorting::handler))
        .route("/Bubble-Sorting", get(pages::bubble_sorting::handler))
        .route("/Pointer", get(pages::pointer::handler))
        .route("/Distribusi-Normal", get(pages::statistics::handler))
        .route("/Pohon", get(pages::tree::handler))
        .route("/Jaringan-Komputer", get(pages::network::handler))
        .route("/Nilai-Eigen", get(pages::eigen::handler))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
