/*
 * Settings
 * Created on 2022-07-05
 */

/***** Setup *****/
/* Imports */
use super::global_settings;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlInputElement};
use yew::{prelude::*, use_context};
#[path = "./underline_text.rs"]
mod underline_text;
#[path = "./widget.rs"]
mod widget;

/***** Glue *****/
#[wasm_bindgen(module = "/src/static/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = getSavePath, catch)]
    pub async fn get_save_path_glue() -> Result<JsValue, JsValue>;
}

/***** Settings component *****/
#[derive(PartialEq, Properties)]
pub struct SettingsProps {
    pub on_update_settings: Callback<global_settings::Settings>,
}

#[function_component(Settings)]
pub fn settings(props: &SettingsProps) -> Html {
    // Settings
    let on_update_settings = props.on_update_settings.clone();
    let settings =
        use_context::<global_settings::Settings>().expect("Could not find settings context");

    // Anti aliasing changed
    let on_anti_aliased_changed = {
        let settings = settings.clone();
        let on_update_settings = on_update_settings.clone();
        Callback::from(move |e: Event| {
            let value = match e.target_dyn_into::<HtmlInputElement>() {
                Some(input) => match input.value().as_str() {
                    "true" => true,
                    "false" => false,
                    _ => settings.anti_aliasing,
                },
                None => settings.anti_aliasing,
            };
            let mut new_settings = settings.clone();
            new_settings.anti_aliasing = value;
            on_update_settings.emit(new_settings);
        })
    };

    let on_get_save_path = {
        let settings = settings.clone();
        Callback::from(move |_| get_image_save_path(settings.clone(), on_update_settings.clone()))
    };

    html! {
        <widget::Widget class="w-80 h-full">
            <p class="text-2xl">{ "Settings" }</p>
            // Anti-aliasing
            <underline_text::UnderlineText>{ "Anti-aliasing" }</underline_text::UnderlineText>
            <label>
                <input
                    onchange={ on_anti_aliased_changed.clone() }
                    type="radio"
                    checked={ settings.anti_aliasing }
                    value="true"
                />
                { " Enabled" }
                <br />
                <input
                    onchange={ on_anti_aliased_changed.clone() }
                    type="radio"
                    checked={ !settings.anti_aliasing }
                    value="false"
                />
                { " Disabled" }
            </label>
            // Save path
            <underline_text::UnderlineText>{ "Save to" }</underline_text::UnderlineText>
            <p>{ settings.save_path }</p>
            <button class="px-2 py-1 w-max text-sm text-black bg-gray-300 rounded-md hover:bg-gray-200" onclick={ on_get_save_path }>{ "Browse" }</button>
        </widget::Widget>
    }
}

/// Get the save image location using JavaScript glue
fn get_image_save_path(
    settings: global_settings::Settings,
    on_update_settings: Callback<global_settings::Settings>,
) {
    spawn_local(async move {
        match get_save_path_glue().await {
            Ok(save_path) => {
                let mut new_settings = settings.clone();
                new_settings.save_path = save_path.as_string().unwrap_or(new_settings.save_path);
                on_update_settings.emit(new_settings);
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
