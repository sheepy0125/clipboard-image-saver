/*
 * Controls
 * Created on 2022-07-05
 */

/***** Setup *****/
/* Imports */
use super::global_settings;
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
}

/***** Controls component *****/
#[function_component(Controls)]
pub fn controls() -> Html {
    // Settings
    let settings =
        use_context::<global_settings::Settings>().expect("Could not find settings context");

    // Save image
    let save_image_path = settings.save_path;
    let on_save_image = {
        match &save_image_path.is_empty() {
            true => Callback::from(move |_| {
                window()
                    .unwrap()
                    .alert_with_message("Save path is empty! Aborting")
                    .unwrap()
            }),
            false => Callback::from(move |_| save_clipboard_image(save_image_path.clone())),
        }
    };

    html! {
        <widget::Widget>
            <p class="text-2xl">{ "Controls" }</p>
            <div class="flex">
                <button onclick={ on_save_image } class="flex-auto p-2 my-2 bg-blue-800 rounded-md hover:bg-blue-700">{ "Save image" }</button>
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
