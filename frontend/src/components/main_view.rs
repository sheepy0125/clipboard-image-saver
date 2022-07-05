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
    html! {
        <div class="flex p-2 min-h-screen">
            <div class="flex-1 mr-2">
                <clipboard_image::ClipboardImage />
            </div>
            <div class="flex flex-col flex-initial">
                <div class="flex-1 mb-2">
                    <settings::Settings />
                </div>
                <div class="flex-initial">
                    <controls::Controls />
                </div>
            </div>
        </div>
    }
}
