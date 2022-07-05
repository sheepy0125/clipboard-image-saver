/*
 * Clipboard reader component
 * Created on 2022-07-02
 */

/***** Setup *****/
/* Imports */
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

/***** Glue *****/
#[wasm_bindgen(module = "/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeReadClipboard, catch)]
    pub async fn read_clipboard() -> Result<JsValue, JsValue>;
}

/***** Clipboard reader component *****/
#[function_component(ClipboardReader)]
pub fn clipboard_reader() -> Html {
    let clipboard = use_state_eq(|| "".to_string());

    {
        let clipboard = clipboard.clone();
        use_effect_with_deps(
            move |_| {
                update_clipboard(clipboard);
                || ()
            },
            (),
        )
    }

    let clipboard = (*clipboard).clone();

    html! {
        <img src={ format!("data:image/png;base64,{}", &clipboard) } />
    }
}

/// Update clipboard state from JavaScript glue
fn update_clipboard(clipboard_state: UseStateHandle<String>) {
    spawn_local(async move {
        match read_clipboard().await {
            Ok(clipboard_contents) => clipboard_state.set(clipboard_contents.as_string().unwrap()),
            Err(e) => {
                // Send an alert
                let window = window().unwrap();
                window.alert_with_message(&e.as_string().unwrap()).unwrap();
            }
        }
    })
}
