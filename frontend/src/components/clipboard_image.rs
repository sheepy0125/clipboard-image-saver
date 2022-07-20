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
#[path = "./widget.rs"]
mod widget;

/***** Glue *****/
#[wasm_bindgen(module = "/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeReadClipboard, catch)]
    pub async fn read_clipboard_glue() -> Result<JsValue, JsValue>;
}

/***** Checkbox grid component *****/
#[derive(PartialEq, Properties)]
struct CheckboxGridComponents {
    children: Children,
}
#[function_component(CheckboxGrid)]
fn checkbox_grid(props: &CheckboxGridComponents) -> Html {
    html! {
        <widget::Widget class="h-full checkerboard">
            { props.children.clone() }
        </widget::Widget>
    }
}

/***** Image display component *****/
#[derive(PartialEq, Properties)]
struct ImageDisplayProps {
    data_url: String,
}
#[function_component(ImageDisplay)]
fn image_display(props: &ImageDisplayProps) -> Html {
    html! {
        <CheckboxGrid>
            <img alt="Image from clipboard" class="border-2 border-white border-opacity-20" src={ props.data_url.clone() } />
        </CheckboxGrid>
    }
}

/***** Clipboard image component *****/
#[function_component(ClipboardImage)]
pub fn clipboard_image() -> Html {
    let clipboard_state = use_state_eq(|| "".to_string());
    {
        let clipboard_state = clipboard_state.clone();
        use_effect_with_deps(
            move |_| {
                update_clipboard(clipboard_state);
                || ()
            },
            (),
        )
    }

    // Control+V listener
    let on_paste = {
        let clipboard_state = clipboard_state.clone();
        Callback::from(move |_| update_clipboard(clipboard_state.clone()))
    };

    let clipboard_base64 = format!("{}", *clipboard_state);

    html! {
        <div onpaste={on_paste} class="h-full">
            <ImageDisplay data_url={ format!("data:image/png;base64,{}", &clipboard_base64) } />
        </div>
    }
}

/// Update clipboard state from JavaScript glue
fn update_clipboard(clipboard_state: UseStateHandle<String>) {
    spawn_local(async move {
        match read_clipboard_glue().await {
            Ok(clipboard_contents) => clipboard_state.set(clipboard_contents.as_string().unwrap()),
            Err(e) => {
                window()
                    .unwrap()
                    .alert_with_message(&e.as_string().unwrap())
                    .unwrap();
            }
        }
    })
}
