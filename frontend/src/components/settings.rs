/*
 * Settings
 * Created on 2022-07-05
 */

/***** Setup *****/
/* Imports */
use yew::prelude::*;
#[path = "./widget.rs"]
mod widget;

/***** Settings component *****/
#[function_component(Settings)]
pub fn settings() -> Html {
    html! {
        <widget::Widget class="h-full">
            <p class="text-2xl">{ "Settings" }</p>
            <p>{ "This should have settings in it." }</p>
            <p>{ "The maximum width should be this. Hello there" }</p>
        </widget::Widget>
    }
}
