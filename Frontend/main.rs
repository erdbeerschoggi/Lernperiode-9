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
#[function_component(App)]
fn app() -> Html {
    // State-Hooks für die Eingabefelder, das Ergebnis und den Speicher
    let result_display = use_state(|| "0.0".to_string());
    let memory_display = use_state(|| "0.0".to_string());
    let num1 = use_state(|| "0".to_string());
    let num2 = use_state(|| "0".to_string());
    let operator = use_state(|| "+".to_string());
    //POST: Berechnung durchführen
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
    // GET: Memory abrufen
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
    // PUT: Memory aktualisieren
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
    // DELETE: Memory löschen
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
    // Event-Handler für die Input-Felder
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
    html! {
    <div class="calculator-card">
        <h1>{ "Rust Taschenrechner" }</h1>
        
        // Display für das Ergebnis
        <div class="display">
            { (*result_display).clone() }
        </div>
        
        // Eingabefelder
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
        
        // POST Button
        <button class="btn btn-primary" onclick={on_calculate}>{ "Berechnen (=)" }</button>
        
        // Memory Sektion
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
fn main() {
    yew::Renderer::<App>::new().render();
}