use yew::prelude::*;
use gloo_net::http::Request;

#[function_component(App)]
fn app() -> Html {
    let backend_response = use_state(|| String::from("BACKEND LADEN AYE AYYE AYYYEEE...."));

    {
        let backend_response = backend_response.clone();
        
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:3000")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                
                backend_response.set(response);
            });
            || ()
        });
    }

    html! {
        <div style="font-family: sans-serif; text-align: center; margin-top: 50px;">
            <h1>{ "Mein Rust Frontend" }</h1>
            <p style="color: green; font-weight: bold;">
                { "Antwort vom Backend: " } { (*backend_response).clone() }
            </p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}