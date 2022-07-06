/*
 * Main view
 * Created on 2022-07-02
 */

/***** Setup *****/
/* Imports */
use yew::prelude::*;
#[path = "./clipboard_image.rs"]
mod clipboard_image;
#[path = "./controls.rs"]
mod controls;
#[path = "./settings.rs"]
mod settings;

/***** Main view *****/
#[function_component(MainView)]
pub fn main_view() -> Html {
    // Right side collapsed
    let right_side_collapsed = use_state_eq(|| true);
    let on_collapsed_toggle_click = {
        let right_side_collapsed = right_side_collapsed.clone();
        Callback::from(move |_| right_side_collapsed.set(!*right_side_collapsed))
    };
    html! {
        <div class="flex p-2 min-h-screen">
            // Image view
            <div class="flex-1">
                <clipboard_image::ClipboardImage />
            </div>
            // Settings and controls
            <div class={
                format!(
                    "ml-2 flex-col flex-initial {}",
                    match *right_side_collapsed {
                        false => "hidden",
                        true => "flex"
                    }
                )
            }>
                <div class="flex-1 mb-2">
                    <settings::Settings />
                </div>
                <div class="flex-initial">
                    <controls::Controls />
                </div>
            </div>
            // Collapse
            <button onclick={on_collapsed_toggle_click} class="ml-2 text-white">
                {
                    match *right_side_collapsed {
                        true => ">",
                        false => "<"
                    }
                }
            </button>
        </div>
    }
}
