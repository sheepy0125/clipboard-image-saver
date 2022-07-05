/*
 * Controls
 * Created on 2022-07-05
 */

/***** Setup *****/
/* Imports */
use yew::prelude::*;
#[path = "./widget.rs"]
mod widget;

/***** Controls component *****/
#[function_component(Controls)]
pub fn controls() -> Html {
    html! {
        <widget::Widget>
            <p class="text-2xl">{ "Controls" }</p>
            <p>{ "This should have controls in it." }</p>
            <p>{ "The maximum width should be this. Hello there" }</p>
        </widget::Widget>
    }
}
