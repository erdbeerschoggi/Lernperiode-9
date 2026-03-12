use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

// Diese Strukturen spiegeln die Daten wider, die wir an das Backend senden und von ihm empfangen
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
    // use_state ist ein Hook, um einen Zustand in der Komponente zu verwalten. Hier speichern wir die Log-Ausgabe.
    let log_output = use_state(|| String::from("Sende Anfragen an das Backend...\n"));

    {
        let log_output = log_output.clone();
        
        // use_effect_with ist ein Hook, um Seiteneffekte zu verwalten. Hier führen wir unsere HTTP-Anfragen aus, sobald die Komponente geladen wird.
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let mut protokoll = String::new();

                // POST: Wir senden eine Rechnung (5 + 10)
                let payload = CalcPayload { x: 5.0, y: 10.0, op: "+".to_string() };
                let post_resp = Request::post("http://localhost:3000/api/calculate")
                    .json(&payload).unwrap().send().await.unwrap();
                let post_data: CalcResponse = post_resp.json().await.unwrap();
                protokoll.push_str(&format!("POST (5 + 10): {}\n", post_data.result));

                // PUT: Wir speichern das Ergebnis (15) im Memory
                let memory_payload = MemoryPayload { value: post_data.result };
                let put_resp = Request::put("http://localhost:3000/api/memory")
                    .json(&memory_payload).unwrap().send().await.unwrap();
                let put_data: CalcResponse = put_resp.json().await.unwrap();
                protokoll.push_str(&format!("PUT (Speichere 15): {}\n", put_data.result));

                // GET: Wir rufen den Memory-Wert wieder ab
                let get_resp = Request::get("http://localhost:3000/api/memory")
                    .send().await.unwrap();
                let get_data: CalcResponse = get_resp.json().await.unwrap();
                protokoll.push_str(&format!("GET (Speicher abrufen): {}\n", get_data.result));

                // DELETE: Wir löschen den Speicher (setzt ihn auf 0)
                let delete_resp = Request::delete("http://localhost:3000/api/memory")
                    .send().await.unwrap();
                let delete_data: CalcResponse = delete_resp.json().await.unwrap();
                protokoll.push_str(&format!("DELETE (Speicher löschen): {}\n", delete_data.result));

                // Den gesammelten Text auf dem Bildschirm ausgeben
                log_output.set(protokoll);
            });
            || ()
        });
    }

    // Das absolute Minimum an HTML (nur ein pre-Tag für unformatierten Text)
    html! {
        <div style="font-family: monospace; margin: 20px;">
            <h2>{ "HTTP Methoden Test" }</h2>
            <pre style="background-color: #f4f4f4; padding: 15px; border-radius: 5px;">
                { (*log_output).clone() }
            </pre>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}