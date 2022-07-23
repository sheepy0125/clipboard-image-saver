/*
 * Clipboard reader component
 * Created on 2022-07-02
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
    #[wasm_bindgen(js_name = invokeReadClipboard, catch)]
    pub async fn read_clipboard_glue() -> Result<JsValue, JsValue>;
}

/***** Image display component *****/
#[derive(PartialEq, Properties)]
struct ImageDisplayProps {
    data_url: String,
}
#[function_component(ImageDisplay)]
fn image_display(props: &ImageDisplayProps) -> Html {
    // Settings
    let settings =
        use_context::<global_settings::Settings>().expect("Could not find settings context");

    // Anti aliasing
    let anti_aliasing = settings.anti_aliasing;

    // Dragging
    // The position of the cursor
    let cursor_pos_state = use_state_eq(|| [0, 0]);
    // The position of the mouse cursor with the origin being the top-left point of the image
    let start_dragging_pos_state = use_state_eq(|| [0, 0]);
    // The position that the image should be rendered at, factoring in the difference between
    // the image pos and the start dragging pos. Will only be updated when `dragging_state`
    let display_image_pos_state = use_state_eq(|| [0, 0]);
    // Whether the image is being dragged or not
    let dragging_state = use_state_eq(|| false);

    let start_dragging = {
        let dragging_state = dragging_state.clone();
        let display_image_pos_state = display_image_pos_state.clone();
        let cursor_pos_state = cursor_pos_state.clone();
        let start_dragging_pos_state = start_dragging_pos_state.clone();
        Callback::from(move |event: MouseEvent| {
            dragging_state.set(true);
            let current_image_pos = *display_image_pos_state;
            start_dragging_pos_state.set([
                event.client_x() - current_image_pos[0],
                event.client_y() - current_image_pos[1],
            ]);
            cursor_pos_state.set([event.client_x(), event.client_y()]);
        })
    };
    let dragging = {
        let cursor_pos_state = cursor_pos_state.clone();
        let display_image_pos_state = display_image_pos_state.clone();
        let start_dragging_pos_state = start_dragging_pos_state.clone();
        let dragging_state = dragging_state.clone();
        Callback::from(move |event: MouseEvent| {
            // The actual position that the image is rendered at will have to subtract
            // the relative start drag position

            if !*dragging_state {
                return;
            }

            cursor_pos_state.set([event.x(), event.y()]);

            let display_pos = match [
                &(*cursor_pos_state)[0] - &(*start_dragging_pos_state)[0],
                &(*cursor_pos_state)[1] - &(*start_dragging_pos_state)[1],
            ] {
                [0, 0] => *display_image_pos_state,
                new_pos => new_pos,
            };

            display_image_pos_state.set(display_pos);
        })
    };
    let stop_dragging = {
        let dragging_state = dragging_state.clone();
        Callback::from(move |_| dragging_state.set(false))
    };

    // Zooming
    let image_size_percent_state = use_state_eq(|| 100);

    let on_zoom_in = {
        let image_size_percent_state = image_size_percent_state.clone();
        let new_size_percent = *image_size_percent_state + 10;
        Callback::from(move |_| image_size_percent_state.set(new_size_percent))
    };
    let on_zoom_out = {
        let image_size_percent_state = image_size_percent_state.clone();
        let mut new_size_percent = *image_size_percent_state - 10;
        // Disallow negative percentages
        if new_size_percent <= 0 {
            new_size_percent = 0;
        }
        Callback::from(move |_| image_size_percent_state.set(new_size_percent))
    };
    let on_zoom_reset = {
        // Resetting the zoom will also reset the position
        let image_size_percent_state = image_size_percent_state.clone();
        let display_image_pos_state = display_image_pos_state.clone();
        Callback::from(move |_| {
            display_image_pos_state.set([0, 0]);
            image_size_percent_state.set(100);
        })
    };

    html! {
        <widget::Widget
            class={"
                bg-widget border-2 bg-gray-700 bg-opacity-80 border-pink-200 h-full
                text-blue-200 rounded-lg shadow-lg overflow-hidden checkerboard
                select-none
            "}
            onmousemove={ dragging }
            onmouseup={ stop_dragging }
        >
            // Debug information (uncomment to show)
            /*
            <p>
                {
                    format!(
                        "image_size_percent {:?} dragging {:?} cursor_pos {:?}
                        display_image_pos {:?} start_dragging_pos {:?}",
                        *image_size_percent_state, *dragging_state, *cursor_pos_state,
                        *display_image_pos_state, *start_dragging_pos_state
                    )
                }
            </p>
            */
            <div class="w-full max-w-max"> // for zoom percentages based on image width
                // TODO: anti-aliasing toggle
                <img
                    onmousedown={ start_dragging }
                    alt="Image from clipboard"
                    id="clipboard-image"
                    draggable="false"
                    class="relative border-2 border-white border-opacity-20 cursor-move"
                    src={ props.data_url.clone() }
                    style={
                        format!(
                            "width: {}%; left: {}px; top: {}px; {}",
                            *image_size_percent_state, &(*display_image_pos_state)[0], &(*display_image_pos_state)[1],
                            match anti_aliasing {
                                true => "",
                                false => "image-rendering: pixelated;",
                            }
                        )
                    }
                />
            </div>
            <div class="flex absolute bottom-0 left-0 m-4">
                // https://heroicons.com/
                // minus-circle outline
                <svg
                    onclick={ on_zoom_out }
                    id="zoom-out-button"
                    class="w-6 h-6 cursor-pointer"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M15 12H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                </svg>
                // search-circle
                <svg
                    onclick={ on_zoom_reset }
                    id="zoom-reset-button"
                    class="w-6 h-6 cursor-pointer"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M8 16l2.879-2.879m0 0a3 3 0 104.243-4.242 3 3 0 00-4.243 4.242zM21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                </svg>
                // plus-circle outline
                <svg
                    onclick={ on_zoom_in }
                    id="zoom-in-button"
                    class="w-6 h-6 cursor-pointer"
                    fill="none" viewBox="0 0 24 24"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                </svg>
            </div>
        </widget::Widget>
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
