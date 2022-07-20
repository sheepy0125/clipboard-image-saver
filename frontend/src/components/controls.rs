/*
 * Controls
 * Created on 2022-07-05
 */

/***** Setup *****/
/* Imports */
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;
#[path = "./widget.rs"]
mod widget;

/***** Glue *****/
#[wasm_bindgen(module = "/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeSaveImage, catch)]
    pub async fn save_image(path: String) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_name = getSavePath, catch)]
    pub async fn get_save_path() -> Result<JsValue, JsValue>;
}

/***** Controls component *****/
#[function_component(Controls)]
pub fn controls() -> Html {
    let save_path = use_state_eq(|| "image.png".to_string());

    let on_get_save_path = {
        let save_path = save_path.clone();
        Callback::from(move |_| get_image_save_path(save_path.clone()))
    };
    let on_save = {
        let save_path = save_path.clone();
        Callback::from(move |_| save_clipboard_to_image(format!("{}", *save_path)))
    };

    html! {
        <widget::Widget>
            <p class="text-2xl">{ "Controls" }</p>
            <p>{ "VVV will be moved to settings widget, but no yewdux yet" }</p>
            <label>
                <span>{ "Path to save" }</span>
                <button onclick={on_get_save_path}>{ "Browse" }</button>
            </label>
            <br />
            <div class="flex">
                <button onclick={on_save} class="flex-auto p-2 my-2 bg-blue-800 rounded-sm hover:bg-blue-700">{ "Save image" }</button>
            </div>
        </widget::Widget>
    }
}

/// Save clipboard to file from JavaScript glue
fn save_clipboard_to_image(path: String) {
    spawn_local(async move {
        match save_image(path.clone()).await {
            Ok(_) => {
                window()
                    .unwrap()
                    .alert_with_message(&format!("Saved clipboard image to {}", path).as_str())
                    .unwrap();
            }
            Err(e) => {
                window()
                    .unwrap()
                    .alert_with_message(&e.as_string().unwrap())
                    .unwrap();
            }
        }
    })
}

/// Get the save image location using JavaScript glue
fn get_image_save_path(save_image_state: UseStateHandle<String>) {
    spawn_local(async move {
        match get_save_path().await {
            Ok(save_path) => save_image_state.set(save_path.as_string().unwrap()),
            Err(e) => {
                window()
                    .unwrap()
                    .alert_with_message(&e.as_string().unwrap())
                    .unwrap();
            }
        }
    })
}
