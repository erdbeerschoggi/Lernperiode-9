---
title: My Tutorial
---

# Goal

In this tutorial, you will learn how to create a full-stack calculator that is able to do imple math in the browser.

# Previous Knowledge

We'll assume you have some basic programming knowledge and habe worked with Rust before.

# What you'll learn

You will learn how to build a calculato where the backend is written with Axum and the frontend is written wth Yew. While doing this tutorial, you will also learn essential concepts like how to connect HTML/CSS in WebAssembly and how the frontend and backend communicate with eachother.

# Tutorial

In order for our calculator to work, we need to divide the project into two parts: A backend that performs the calculations and a frontend that displays the calculator to the user in the browser.

# The Backend:
First create a folder for your backend and adjust the Cargo.toml so Rust knows which tools we need:

```rust
[package]
name = "Axum_Rust"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tower-http = { version = "0.5", features = ["cors"] }
```
Now we open the main.rs of the backend and start with the imports and the data srtuctures. 
```rust
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

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

type SharedState = Arc<Mutex<f64>>;
```
Lets break this down:
Imports: we load ***axum*** for the web server and ***serde*** so the server can understand text sent from the frontend.

Structs(CalcRequest, CalcResponse): These are our data blueprirnts. When the frontend wants to calculate something it sends ***x***,***y*** and the operator ***op***.

SharedState: This is our memory. Since multiple requests can arrive at the same time, ***Mutex*** protects our memory from accidentally being overwritten twice simultaneously.


Now we write the actual logic for our calculator right below that:
```rust
//POST
async fn calculate(Json(payload): Json<CalcRequest>) -> Json<CalcResponse> {
    let x = payload.x;
    let y = payload.y;
    let op = payload.op.trim();

    let result = match op {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => {
            if y == 0.0 { 0.0 } else { x / y } 
        }
        _ => 0.0,
    };

    Json(CalcResponse { result })
}
```
Lets break this down:
***match op***: Here we check which math operator the use chose.
***if y == 0.0***: This is a small safety measure that prevents our program from crashing if someone tries to divide by zero.

Now we add the memory functions
```rust
//GET
async fn get_memory(State(state): State<SharedState>) -> Json<CalcResponse> {
    let memory = *state.lock().unwrap();
    Json(CalcResponse { result: memory })
}
//PUT
async fn update_memory(State(state): State<SharedState>, Json(payload): Json<MemoryRequest>) -> Json<CalcResponse> {
    let mut memory = state.lock().unwrap();
    *memory = payload.value;
    Json(CalcResponse { result: *memory })
}
//DELETE
async fn delete_memory(State(state): State<SharedState>) -> Json<CalcResponse> {
    let mut memory = state.lock().unwrap();
    *memory = 0.0;
    Json(CalcResponse { result: *memory })
}
```
These three functions read, update and clear out ***SharedState*** memory.

Finally, the backend just needs the main function to start the server. This is what you need to add to the end of your file:
```rust
#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();
    let app_state: SharedState = Arc::new(Mutex::new(0.0)); 

    let app = Router::new()
        .route("/api/calculate", post(calculate))
        .route("/api/memory", get(get_memory).put(update_memory).delete(delete_memory))
        .with_state(app_state) 
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Backend läuft auf http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
```
Important: The ***let cors = CorsLayer::permissive();*** is essential. It allows our frontend to talk to our server Without this, the browser would block the connection.


# Part 2: The Frontend
Now create a second folder for your frontend. Lets define the Cargo.toml first:
```rust
[package]
name = "frontend"
version = "0.1.0"
edition = "2024"

[dependencies]
yew = { version = "0.21", features = ["csr"] }
gloo-net = "0.3"
wasm-bindgen-futures = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
web-sys = { version = "0.3", features = ["HtmlInputElement", "HtmlSelectElement"] }
```

Next we need an ndex.html in the main folder of your frontend. This loads our website:
```rust
<!DOCTYPE html>
<html lang="de">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link data-trunk rel="css" href="style.css" />
    <title>Mein Rust Taschenrechner</title>
  </head>
  <body>
  </body>
</html>
```
Lets break this down:
***data-trunk***: This is extremely important! Wr use a tool called "Trunk" to build our frontend. If you only use a normal <link> tag here, your CSS will be ignored. The ***data-trunk*** attribute forces Trunk to inlcude the CSS file in your finished project.
***Then you can create the style.css file and create your design.***

Now open the main.rs from the frontend folder. First we define the improts and structs:
```rust
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
#[derive(Serialize)]
struct CalcPayload {
    x: f64,
    y: f64,
    op: String,
}
#[derive(Deserialize)]
struct CalcResponse {
    result: f64,
}
#[derive(Serialize)]
struct MemoryPayload {
    value: f64,
}
```

Now we create the main component and initialize the "State". This is the state of our app that changes when you type:
```rust
#[function_component(App)]
fn app() -> Html {
    let result_display = use_state(|| "0.0".to_string());
    let memory_display = use_state(|| "0.0".to_string());
    let num1 = use_state(|| "0".to_string());
    let num2 = use_state(|| "0".to_string());
    let operator = use_state(|| "+".to_string());
```
Right below that, we define what should happen when the user clicks the calculate button:
```rust
// POST
    let on_calculate = {
        let num1 = num1.clone();
        let num2 = num2.clone();
        let operator = operator.clone();
        let result_display = result_display.clone();
        Callback::from(move |_| {
            let x: f64 = num1.parse().unwrap_or(0.0);
            let y: f64 = num2.parse().unwrap_or(0.0);
            let op = (*operator).clone();
            let result_display = result_display.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let payload = CalcPayload { x, y, op };
                if let Ok(response) = Request::post("http://localhost:3000/api/calculate")
                    .json(&payload)
                    .unwrap()
                    .send()
                    .await
                {
                    if let Ok(data) = response.json::<CalcResponse>().await {
                        result_display.set(data.result.to_string());
                    }
                }
            });
        })
    };
```
Lets break this down:
***Request::post(...)***: This is where the frontend calls your backend. It sends the numbers as a package and waits for the answer.

Now we add the three actions for the memory(Read,Save,Delete):
```rust
// GET
    let on_get_memory = {
        let memory_display = memory_display.clone();
        Callback::from(move |_| {
            let memory_display = memory_display.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(response) = Request::get("http://localhost:3000/api/memory")
                    .send()
                    .await
                {
                    if let Ok(data) = response.json::<CalcResponse>().await {
                        memory_display.set(data.result.to_string());
                    }
                }
            });
        })
    };
    // PUT
    let on_save_memory = {
        let result_display = result_display.clone();
        let memory_display = memory_display.clone();
        Callback::from(move |_| {
            let current_result: f64 = result_display.parse().unwrap_or(0.0);
            let memory_display = memory_display.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let payload = MemoryPayload { value: current_result };
                if let Ok(response) = Request::put("http://localhost:3000/api/memory")
                    .json(&payload)
                    .unwrap()
                    .send()
                    .await
                {
                    if let Ok(data) = response.json::<CalcResponse>().await {
                        memory_display.set(data.result.to_string());
                    }
                }
            });
        })
    };
    // DELETE
    let on_delete_memory = {
        let memory_display = memory_display.clone();
        Callback::from(move |_| {
            let memory_display = memory_display.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(response) = Request::delete("http://localhost:3000/api/memory")
                    .send()
                    .await
                {
                    if let Ok(data) = response.json::<CalcResponse>().await {
                        memory_display.set(data.result.to_string());
                    }
                }
            });
        })
    };
```

Next we need the Event Handlers. These detect when the user types a number into a field or changes the operator:
```rust
    let oninput_num1 = {
        let num1 = num1.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            num1.set(input.value());
        })
    };
    let oninput_num2 = {
        let num2 = num2.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            num2.set(input.value());
        })
    };
    let onchange_op = {
        let operator = operator.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            operator.set(select.value());
        })
    };
```

Now we complete our app function by defining the HTML for the user interface
```rust
html! {
    <div class="calculator-card">
        <h1>{ "Rust Taschenrechner" }</h1>
        
        <div class="display">
            { (*result_display).clone() }
        </div>
        
        <div class="input-group">
            <input type="number" value={(*num1).clone()} oninput={oninput_num1} />
            <select onchange={onchange_op}>
                <option value="+">{ "+" }</option>
                <option value="-">{ "-" }</option>
                <option value="*">{ "*" }</option>
                <option value="/">{ "/" }</option>
            </select>
            <input type="number" value={(*num2).clone()} oninput={oninput_num2} />
        </div>
        
        //Post
        <button class="btn btn-primary" onclick={on_calculate}>{ "Berechnen (=)" }</button>
        
        // Memory
        <div class="memory-display">
            { "Speicher (M): " } <strong>{ (*memory_display).clone() }</strong>
        </div>
        
        <div class="memory-group">
            <button class="btn btn-memory" onclick={on_get_memory} title="GET Request">{ "M Lesen" }</button>
            <button class="btn btn-memory" onclick={on_save_memory} title="PUT Request">{ "M Speichern" }</button>
            <button class="btn btn-danger" onclick={on_delete_memory} title="DELETE Request">{ "M Löschen" }</button>
        </div>
    </div>
}
}
```

Finally all the frontend needs is the start function at the very end of the file:
```rust
fn main() {
    yew::Renderer::<App>::new().render();
}
```

# Result
Now in order for our code to run, we need to open two terminals:
1. In your backend folder, type the command: ***cargo run***
2. In your frontend folder, type the command: ***trunk serve***

Now open your browser and you will see your working calculator!

# What could go wrong?
1. HTML and CSS arent connecting: A mistake I made at the beginning was forgettng to add the data-trunk attribute in the <link> tag of the HTML file. So make sure this line is written so you can link the style.css and html -> <link data-trunk rel="css" href="style.css" />.
2. Frontend cant communicate with the Backend: IF your calculotr looks good but nothing hppens when you click "=", its often due to your browsers security. Make sure that you included CorsLayer::permissive() in your backends main function and attached it to your router with .layer(cors).
3. Wrong URLs in the Frontend: Pay close attention to ensure your frontend is sending the exact request to the correct URL. A typo in the web address at Request::post will cause the request to vanish into nowhere.
