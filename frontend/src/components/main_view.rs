/*
 * Main view
 * Created on 2022-07-02
 */

/***** Setup *****/
/* Imports */
use yew::prelude::*;
#[path="./clipboard_get.rs"]
mod clipboard_get;

/***** Main view *****/
#[function_component(MainView)]
pub fn main_view() -> Html {
    html! {
        <clipboard_get::ClipboardReader />
    }
}
