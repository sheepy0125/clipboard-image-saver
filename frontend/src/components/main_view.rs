/*
 * Main view
 * Created on 2022-07-02
 */

/***** Setup *****/
/* Imports */
use yew::prelude::*;
#[path = "./clipboard_image.rs"]
mod clipboard_image;

/***** Main view *****/
#[function_component(MainView)]
pub fn main_view() -> Html {
    html! {
        <clipboard_image::ClipboardReader />
    }
}
