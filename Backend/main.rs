use axum ::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};


#[tokio::main]
async fn main() {
    // CORS erlauben, so das die Frontend zugreifen kann
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World! THIS IS THHHHEEEEEEEEEEE BACKEND MESSSAGGEEE AAYEEEEE!!!" }));
    let app = app.layer(cors);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}