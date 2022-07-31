/*
 * Clipboard reader component
 * Created on 2022-07-02
 */

/***** Setup *****/
/* Imports */
use super::global_settings;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlImageElement};
use yew::{prelude::*, virtual_dom::AttrValue};
#[path = "./widget.rs"]
mod widget;

/***** Glue *****/
#[wasm_bindgen(module = "/src/static/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeReadClipboard, catch)]
    pub async fn read_clipboard_glue() -> Result<JsValue, JsValue>;
}

/***** Image display component *****/
#[derive(PartialEq, Properties, Clone)]
struct ImageDisplayProps {
    data_url: AttrValue,
    reset_zoom: bool,
}
#[function_component(ImageDisplay)]
fn image_display(props: &ImageDisplayProps) -> Html {
    // Settings
    let settings =
        use_context::<global_settings::Settings>().expect("Could not find settings context");

    // Anti aliasing
    let anti_aliasing = settings.anti_aliasing;

    // Image ref
    let image_ref = use_node_ref();
    let image_ref_element = image_ref
        .cast::<HtmlImageElement>()
        .unwrap_or_else(|| HtmlImageElement::new().unwrap());

    // Dragging
    // The position of the cursor
    let cursor_pos_ref = use_mut_ref(|| [0, 0]);
    // The position of the mouse cursor with the origin being the top-left point of the image
    let start_dragging_pos_ref = use_mut_ref(|| [0, 0]);
    // The position that the image should be rendered at, factoring in the difference between
    // the image pos and the start dragging pos. Will only be updated when `dragging_ref` does
    let display_image_pos_ref = use_mut_ref(|| [0, 0]);
    // Whether the image is being dragged or not
    let dragging_ref = use_mut_ref(|| false);
    let dragging_style_update = {
        let style = image_ref_element.style();
        let display_image_pos_ref = display_image_pos_ref.clone();
        move || {
            let display_pos = *display_image_pos_ref.borrow();
            style
                .set_property("left", format!("{}px", display_pos[0]).as_str())
                .unwrap();
            style
                .set_property("top", format!("{}px", display_pos[1]).as_str())
                .unwrap();
        }
    };
    let start_dragging = {
        let dragging_ref = dragging_ref.clone();
        let display_image_pos_ref = display_image_pos_ref.clone();
        let cursor_pos_ref = cursor_pos_ref.clone();
        let start_dragging_pos_ref = start_dragging_pos_ref.clone();
        Callback::from(move |event: MouseEvent| {
            *dragging_ref.borrow_mut() = true;
            let current_image_pos = *display_image_pos_ref.borrow();
            *start_dragging_pos_ref.borrow_mut() = [
                event.client_x() - current_image_pos[0],
                event.client_y() - current_image_pos[1],
            ];
            *cursor_pos_ref.borrow_mut() = [event.client_x(), event.client_y()];
        })
    };
    let dragging = {
        let display_image_pos_ref = display_image_pos_ref.clone();
        let dragging_ref = dragging_ref.clone();
        let dragging_style_update = dragging_style_update.clone();
        Callback::from(move |event: MouseEvent| {
            // The actual position that the image is rendered at will have to subtract
            // the relative start drag position
            if !*dragging_ref.borrow() {
                return;
            }

            // Update cursor position to avoid first "frame" of dragging in the
            // wrong place
            *cursor_pos_ref.borrow_mut() = [event.x(), event.y()];

            // Get new display position
            let cursor_pos = *cursor_pos_ref.borrow();
            let start_dragging_pos = *start_dragging_pos_ref.borrow();
            let display_image_pos = *display_image_pos_ref.borrow();

            let display_pos = match [
                cursor_pos[0] - start_dragging_pos[0],
                cursor_pos[1] - start_dragging_pos[1],
            ] {
                [0, 0] => display_image_pos,
                new_pos => new_pos,
            };

            *display_image_pos_ref.borrow_mut() = display_pos;

            dragging_style_update();
        })
    };
    let stop_dragging = Callback::from(move |_| {
        *dragging_ref.borrow_mut() = false;
    });

    // Zooming
    let image_size_percent_ref = use_mut_ref(|| 100);
    let zoom_style_update = {
        let image_size_percent_ref = image_size_percent_ref.clone();
        move || {
            let style = image_ref_element.style();
            style
                .set_property(
                    "width",
                    format!("{}%", *image_size_percent_ref.borrow()).as_str(),
                )
                .unwrap();
        }
    };
    let on_zoom_in = {
        let image_size_percent_ref = image_size_percent_ref.clone();
        let zoom_style_update = zoom_style_update.clone();
        Callback::from(move |_| {
            *image_size_percent_ref.borrow_mut() += settings.zoom_by;
            zoom_style_update();
        })
    };
    let on_zoom_out = {
        let image_size_percent_ref = image_size_percent_ref.clone();
        let zoom_style_update = zoom_style_update.clone();
        Callback::from(move |_| {
            let mut new_size_percent = *image_size_percent_ref.borrow() - settings.zoom_by;
            // Disallow negative percentages
            if new_size_percent <= 0 {
                new_size_percent = 0;
            }
            *image_size_percent_ref.borrow_mut() = new_size_percent;
            zoom_style_update();
        })
    };
    let on_zoom_reset = {
        let zoom_style_update = zoom_style_update.clone();
        // Resetting the zoom will also reset the position
        let dragging_style_update = dragging_style_update.clone();
        Callback::from(move |_| {
            *display_image_pos_ref.borrow_mut() = [0, 0];
            *image_size_percent_ref.borrow_mut() = 100;
            zoom_style_update();
            dragging_style_update();
        })
    };

    // Reset zoom if props require
    let reset_zoom = props.reset_zoom;
    if reset_zoom {
        // Call the reset zoom callback with a made-up mouse up event
        let fake_mouse_event = MouseEvent::new("mouseup").unwrap();
        on_zoom_reset.emit(fake_mouse_event);
    }

    // If there was a rerender, the image style may not be updated properly
    // So update it!
    use_effect({
        move || {
            zoom_style_update();
            dragging_style_update();
            || ()
        }
    });

    html! {
        <widget::Widget
            class={"
                border-2 bg-gray-700 bg-opacity-80 border-pink-200 text-blue-200
                rounded-lg shadow-lg overflow-hidden checkerboard select-none
                h-full w-full
            "}
            // style="width: 1520px !important"
            onmousemove={ dragging }
            onmouseup={ stop_dragging }
        >
            <div class="w-full">
                <div class="w-max">
                    <img
                        onmousedown={ start_dragging }
                        ref={ image_ref }
                        alt="Image from clipboard"
                        id="clipboard-image"
                        draggable="false"
                        class="relative border-2 border-white border-opacity-20 cursor-move"
                        src={ props.data_url.clone() }
                        style={
                            (match anti_aliasing {
                                true => "",
                                false => "image-rendering: pixelated; image-rendering: crisp-edges;",
                            }).to_string()
                        }
                    />
                </div>
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
#[derive(PartialEq, Properties)]
pub struct ClipboardImageProps {
    pub should_update_clipboard: bool,
}
#[function_component(ClipboardImage)]
pub fn clipboard_image(props: &ClipboardImageProps) -> Html {
    // Settings
    let settings =
        use_context::<global_settings::Settings>().expect("Could not find settings context");

    // Clipboard data
    let clipboard_state = use_state_eq(|| AttrValue::from("".to_owned()));

    // Pasting
    let should_update_clipboard = props.should_update_clipboard;
    {
        let clipboard_state = clipboard_state.clone();
        use_effect_with_deps(
            move |_| {
                if settings.auto_paste || should_update_clipboard {
                    update_clipboard(clipboard_state);
                }
                || ()
            },
            (settings.auto_paste, should_update_clipboard),
        )
    }

    html! {
        <ImageDisplay
            reset_zoom={ should_update_clipboard }
            data_url={ AttrValue::from(format!("data:image/png;base64,{}", *clipboard_state)) }
        />
    }
}

/// Update clipboard state from JavaScript glue
fn update_clipboard(clipboard_state: UseStateHandle<AttrValue>) {
    spawn_local(async move {
        match read_clipboard_glue().await {
            Ok(clipboard_contents) => {
                clipboard_state.set(AttrValue::from(clipboard_contents.as_string().unwrap()))
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
