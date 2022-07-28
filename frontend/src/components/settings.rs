/*
 * Settings
 * Created on 2022-07-05
 */

/***** Setup *****/
/* Imports */
use super::global_settings;
use std::{path::PathBuf, str::FromStr};
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlInputElement, HtmlSelectElement};
use yew::{prelude::*, use_context};
#[path = "./underline_text.rs"]
mod underline_text;
#[path = "./widget.rs"]
mod widget;

/***** Glue *****/
#[wasm_bindgen(module = "/src/static/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeGetSavePath, catch)]
    pub async fn get_save_path_glue(format: String) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_name = invokeSaveSettings, catch)]
    pub async fn save_settings_glue(settings_text: String) -> Result<JsValue, JsValue>;
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

    // Anti aliasing
    let on_anti_aliased_changed = {
        let settings = settings.clone();
        let on_update_settings = on_update_settings.clone();
        Callback::from(move |event: Event| {
            let value = match event.target_dyn_into::<HtmlInputElement>() {
                Some(input) => match input.value().as_str() {
                    "true" => true,
                    "false" => false,
                    _ => settings.anti_aliasing,
                },
                None => !settings.anti_aliasing,
            };
            let mut new_settings = settings.clone();
            new_settings.anti_aliasing = value;
            on_update_settings.emit(new_settings);
        })
    };

    // Auto paste
    let on_auto_paste_changed = {
        let settings = settings.clone();
        let on_update_settings = on_update_settings.clone();
        Callback::from(move |event: Event| {
            let value = match event.target_dyn_into::<HtmlInputElement>() {
                Some(input) => match input.value().as_str() {
                    "true" => true,
                    "false" => false,
                    _ => settings.auto_paste,
                },
                None => !settings.auto_paste,
            };
            let mut new_settings = settings.clone();
            new_settings.auto_paste = value;
            on_update_settings.emit(new_settings);
        })
    };

    // Image save path
    let on_get_save_path = {
        let settings = settings.clone();
        let on_update_settings = on_update_settings.clone();

        Callback::from(move |_| {
            get_image_save_path(settings.clone(), on_update_settings.clone());
        })
    };

    // Image save format
    let on_save_format_change = {
        let settings = settings.clone();
        let on_update_settings = on_update_settings.clone();

        Callback::from(move |event: Event| {
            let save_format = match event.target_dyn_into::<HtmlSelectElement>() {
                Some(input) => global_settings::SaveFormat::from_str(input.value().as_str())
                    .unwrap_or_else(|_| settings.save_format.clone()),
                None => settings.save_format.clone(),
            };
            let mut new_settings = settings.clone();
            new_settings.save_format = save_format;

            // Also update the file extension for the save path
            let extension = new_settings.save_format.to_string().to_lowercase();
            let mut save_path_buf = PathBuf::from(&new_settings.save_path);
            save_path_buf.set_extension(extension);
            new_settings.save_path = save_path_buf.to_string_lossy().to_string();

            on_update_settings.emit(new_settings);
        })
    };
    // When the save format changes, that doesn't mean that the select tag was changed
    // (e.g. when the settings load from the file)
    // This will update the select tag when the settings update
    let save_format_select_ref = use_node_ref();
    {
        let save_format_select_ref = save_format_select_ref.clone();
        use_effect_with_deps(
            move |selection| {
                if let Some(element) = save_format_select_ref.cast::<HtmlSelectElement>() {
                    let format_string = selection.to_string();
                    element.set_value(format_string.as_str())
                }
                || {}
            },
            settings.save_format.clone(),
        )
    };
    // Same thing with the zoom by slider
    let zoom_by_slider_ref = use_node_ref();
    {
        let zoom_by_slider_ref = zoom_by_slider_ref.clone();
        let zoom_by = settings.zoom_by.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(element) = zoom_by_slider_ref.cast::<HtmlInputElement>() {
                    element.set_value(format!("{}", zoom_by).as_str())
                }
                || {}
            },
            settings.save_format.clone(),
        )
    };

    // Zoom by range changed
    let zoom_by_range = {
        let settings = settings.clone();
        let on_update_settings = on_update_settings.clone();
        Callback::from(move |event: InputEvent| {
            let zoom_by = match event.target_dyn_into::<HtmlInputElement>() {
                Some(input) => input
                    .value()
                    .parse::<i32>()
                    .unwrap_or_else(|_| settings.zoom_by.clone()),
                None => settings.zoom_by.clone(),
            };
            let mut new_settings = settings.clone();
            new_settings.zoom_by = zoom_by;
            on_update_settings.emit(new_settings);
        })
    };

    // Save
    let on_save = {
        let settings = settings.clone();
        Callback::from(move |_| {
            save_settings(settings.clone()).unwrap();
        })
    };

    // Reset
    let on_reset = {
        let default_settings = global_settings::Settings::default();
        let on_update_settings = on_update_settings.clone();
        Callback::from(move |_| {
            on_update_settings.emit(default_settings.clone());
        })
    };

    html! {
        <widget::Widget class="flex flex-col w-80 h-full">
            // Settings toggles
            <div class="flex-1">
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
                // Auto paste
                <underline_text::UnderlineText>{ "Auto paste" }</underline_text::UnderlineText>
                <label>
                    <input
                        onchange={ on_auto_paste_changed.clone() }
                        type="radio"
                        checked={ settings.auto_paste }
                        value="true"
                    />
                    { " Enabled" }
                    <br />
                    <input
                        onchange={ on_auto_paste_changed.clone() }
                        type="radio"
                        checked={ !settings.auto_paste }
                        value="false"
                    />
                    { " Disabled" }
                </label>
                // Save path
                <underline_text::UnderlineText>{ "Save to" }</underline_text::UnderlineText>
                <p>{ settings.save_path }</p>
                <button
                    onclick={ on_get_save_path }
                    class="px-2 py-1 w-max text-sm text-black bg-gray-300 rounded-md hover:bg-gray-200"
                >
                    { "Browse" }
                </button>
                // Save format
                <underline_text::UnderlineText>{ "Save format" }</underline_text::UnderlineText>
                <select
                    class="text-sm text-black"
                    onchange={ on_save_format_change.clone() }
                    ref={ save_format_select_ref }
                >
                    {
                        global_settings::SaveFormat::iter().map(|format| {
                            let string_format = format.to_string();
                            html! {
                                <option
                                    key={ string_format.clone() }
                                    value={ string_format.clone() }
                                    selected={ format == settings.save_format }
                                >
                                    { string_format }
                                </option>
                            }
                        }).collect::<Html>()
                    }
                </select>
                // Zoom by
                <underline_text::UnderlineText>{ "Zoom by" }</underline_text::UnderlineText>
                <p>{
                    format!(
                        "{}%",
                        match zoom_by_slider_ref.cast::<HtmlInputElement>() {
                            Some(element) => element.value(),
                            None => settings.zoom_by.to_string()
                        }
                    )
                }</p>
                <input oninput={ zoom_by_range } ref={ zoom_by_slider_ref } type="range" min=1 max=100 />
            </div>

            // Controls
            <div class="flex flex-initial gap-2 my-2 w-full">
                // Reset
                <button onclick={ on_reset } class="p-2 w-full bg-blue-800 rounded-md hover:bg-blue-700">
                    { "Reset settings" }
                </button>
                // Save
                <button onclick={ on_save } class="p-2 w-full bg-blue-800 rounded-md hover:bg-blue-700">
                    { "Save settings" }
                </button>
            </div>
        </widget::Widget>
    }
}

/// Get the save image location using JavaScript glue
fn get_image_save_path(
    settings: global_settings::Settings,
    on_update_settings: Callback<global_settings::Settings>,
) {
    spawn_local(async move {
        // Get save formats
        let save_format = settings.save_format.to_string();

        match get_save_path_glue(save_format).await {
            Ok(save_path) => {
                let mut new_settings = settings.clone();
                new_settings.save_path = save_path
                    .as_string()
                    .unwrap_or_else(|| new_settings.save_path);
                on_update_settings.emit(new_settings);
            }
            Err(e) => {
                window()
                    .unwrap()
                    .alert_with_message(&e.as_string().unwrap_or_else(|| {
                        format!("Failed to get the save path, but no reason was provided")
                    }))
                    .unwrap();
            }
        }
    })
}

/// Save settings using JavaScript glue
fn save_settings(settings: global_settings::Settings) -> Result<(), ()> {
    // Serialize
    let serialized_data = match serde_json::to_string(&settings) {
        Ok(serialized_data) => serialized_data,
        Err(_) => {
            window()
                .unwrap()
                .alert_with_message("Failed to serialize settings")
                .unwrap();
            return Err(());
        }
    };

    spawn_local(async move {
        match save_settings_glue(serialized_data).await {
            Ok(_) => window()
                .unwrap()
                .alert_with_message("Saved settings")
                .unwrap(),
            Err(e) => window()
                .unwrap()
                .alert_with_message(&e.as_string().unwrap())
                .unwrap(),
        }
    });

    Ok(())
}
