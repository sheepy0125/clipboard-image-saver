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
#[wasm_bindgen(module = "/src/static/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeSaveImage, catch)]
    pub async fn save_image_glue(path: String) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_name = getSavePath, catch)]
    pub async fn get_save_path_glue() -> Result<JsValue, JsValue>;
}

/***** Controls component *****/
#[function_component(Controls)]
pub fn controls() -> Html {
    let save_path_state = use_state_eq(|| "".to_string());

    let on_get_save_path = {
        let save_path_state = save_path_state.clone();
        Callback::from(move |_| get_image_save_path(save_path_state.clone()))
    };
    let on_save_image = {
        let save_path_state = save_path_state.clone();
        let save_path = format!("{}", *save_path_state);
        match &save_path.is_empty() {
            true => Callback::from(move |_| {
                window()
                    .unwrap()
                    .alert_with_message("Save path is empty! Aborting")
                    .unwrap()
            }),
            false => Callback::from(move |_| save_clipboard_image(save_path.clone())),
        }
    };

    html! {
        <widget::Widget>
            <p class="text-2xl">{ "Controls" }</p>
            <p>{ "VVV will be moved to settings widget" }<br />{" but no yewdux yet "}</p>
            <label>
                <span>{ "Path to save" }</span>
                <button onclick={on_get_save_path}>{ "Browse" }</button>
            </label>
            <br />
            <div class="flex">
                <button onclick={on_save_image} class="flex-auto p-2 my-2 bg-blue-800 rounded-sm hover:bg-blue-700">{ "Save image" }</button>
            </div>
        </widget::Widget>
    }
}

/// Save clipboard to file from JavaScript glue
fn save_clipboard_image(path: String) {
    spawn_local(async move {
        match save_image_glue(path.clone()).await {
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
fn get_image_save_path(save_path_state: UseStateHandle<String>) {
    spawn_local(async move {
        match get_save_path_glue().await {
            Ok(save_path) => save_path_state.set(save_path.as_string().unwrap()),
            Err(e) => {
                window()
                    .unwrap()
                    .alert_with_message(&e.as_string().unwrap())
                    .unwrap();
            }
        }
    })
}
