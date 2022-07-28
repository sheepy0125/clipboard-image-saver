/*
 * Control button
 * Created on 2022-07-28
 */

/***** Setup *****/
/* Imports */
use web_sys::MouseEvent;
use yew::prelude::*;

/***** Control button component *****/
#[derive(Properties, PartialEq)]
pub struct ControlButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}
#[function_component(ControlButton)]
pub fn control_button(props: &ControlButtonProps) -> Html {
    let onclick = props.onclick.clone();

    html! {
        <button
            { onclick }
            class="p-2 w-full bg-blue-800 rounded-md hover:bg-blue-700"
        >
            { props.children.clone() }
        </button>
    }
}
