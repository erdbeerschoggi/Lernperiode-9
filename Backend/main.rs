use axum::{
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

//Anfrage vom Frontend
#[derive(Deserialize)]
struct CalcRequest {
    x: f64,          
    y: f64,          
    op: String,      
}

// Antwort an das Frontend zurück
#[derive(Serialize)]
struct CalcResponse {
    result: f64,
}

// Umgebaute Main funktion von meinem alten Taschenrechner, damit sie mit Axum funktioniert
async fn calculate(Json(payload): Json<CalcRequest>) -> Json<CalcResponse> {
    let x = payload.x;
    let y = payload.y;
    let op = payload.op.trim();

    let result: f64 = if op == "1" {
        x + y
    } else if op == "2" {
        x - y
    } else if op == "3" {
        x * y
    } else if op == "4" {
        x / y
    } else {
        0.0 // Platzhalter für ungültige Operatoren
    };

    // Anstatt println! geben wir das Ergebnis als JSON zurück
    Json(CalcResponse { result })
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/api/calculate", post(calculate))
        .layer(cors);

    // Server starten
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Backend läuft auf http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}