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
#[path = "./control_button.rs"]
mod control_button;

/***** Glue *****/
#[wasm_bindgen(module = "/src/static/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeSaveImage, catch)]
    pub async fn save_image_glue(path: String, format: String) -> Result<JsValue, JsValue>;
}

/***** Controls component *****/
#[derive(PartialEq, Properties)]
pub struct ControlsProps {
    pub on_should_update_clipboard: Callback<bool>,
}
#[function_component(Controls)]
pub fn controls(props: &ControlsProps) -> Html {
    // Settings
    let settings =
        use_context::<global_settings::Settings>().expect("Could not find settings context");

    // Save image
    let save_image_path = settings.save_path;
    let on_save_image = {
        let save_format = settings.save_format.clone();
        match &save_image_path.is_empty() {
            true => Callback::from(move |_| {
                window()
                    .unwrap()
                    .alert_with_message("Save path is empty! Aborting")
                    .unwrap()
            }),
            false => Callback::from(move |_| {
                save_clipboard_image(save_image_path.clone(), save_format.to_string())
            }),
        }
    };

    // Update clipboard
    let on_should_update_clipboard = props.on_should_update_clipboard.clone();
    let on_update_clipboard = {
        let on_update_clipboard = on_should_update_clipboard.clone();
        Callback::from(move |_| {
            on_update_clipboard.emit(true);
        })
    };

    html! {
        <widget::Widget>
            <p class="text-2xl">{ "Controls" }</p>
            <div class="flex flex-initial gap-2 my-2 w-full">
                // Refresh
                <control_button::ControlButton onclick={ on_update_clipboard }>
                    { "Refresh clipboard" }
                </control_button::ControlButton>
                // Save
                <control_button::ControlButton onclick={ on_save_image }>
                    { "Save image" }
                </control_button::ControlButton>
            </div>
        </widget::Widget>

    }
}

/// Save clipboard to file from JavaScript glue
fn save_clipboard_image(path: String, format: String) {
    spawn_local(async move {
        match save_image_glue(path.clone(), format.clone()).await {
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
