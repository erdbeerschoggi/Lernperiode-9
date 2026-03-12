use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

// Anfrage an Server für Berechnung
#[derive(Deserialize)]
struct CalcRequest {
    x: f64,
    y: f64,
    op: String,
}

#[derive(Serialize)]
struct CalcResponse {
    result: f64,
}

#[derive(Deserialize)]
struct MemoryRequest {
    value: f64,
}

// Braucht man für den gemeinsamen Zugriff auf den Speicher (Memory) zwischen verschiedenen Anfragen
type SharedState = Arc<Mutex<f64>>;

// POST: Berechnung durchführen
async fn calculate(Json(payload): Json<CalcRequest>) -> Json<CalcResponse> {
    let x = payload.x;
    let y = payload.y;
    let op = payload.op.trim();

    let result = match op {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => {
            if y == 0.0 { 0.0 } else { x / y } // Verhindert Crash durch Division durch 0
        }
        _ => 0.0,
    };

    Json(CalcResponse { result })
}

//GET: Aktuellen Wert aus dem Speicher abrufen
async fn get_memory(State(state): State<SharedState>) -> Json<CalcResponse> {
    let memory = *state.lock().unwrap();
    Json(CalcResponse { result: memory })
}

//PUT: Neuen Wert in den Speicher schreiben
async fn update_memory(State(state): State<SharedState>, Json(payload): Json<MemoryRequest>) -> Json<CalcResponse> {
    let mut memory = state.lock().unwrap();
    *memory = payload.value;
    Json(CalcResponse { result: *memory })
}

//DELETE: Speicher zurücksetzen/löschen
async fn delete_memory(State(state): State<SharedState>) -> Json<CalcResponse> {
    let mut memory = state.lock().unwrap();
    *memory = 0.0;
    Json(CalcResponse { result: *memory })
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();
    let app_state: SharedState = Arc::new(Mutex::new(0.0)); 

    let app = Router::new()
        .route("/api/calculate", post(calculate))
        // Hier werden die drei Routen für den Speicher definiert: GET, PUT und DELETE
        .route("/api/memory", get(get_memory).put(update_memory).delete(delete_memory))
        .with_state(app_state) // Gemeinsamer Zustand (Memory) für alle Routen
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Backend läuft auf http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}