/*
 * Text underline component
 * Created on 2022-07-22
 */

/***** Setup *****/
/* Imports */
use yew::prelude::*;

/***** Underline text component *****/
#[derive(Properties, PartialEq)]
pub struct UnderlineTextProps {
    pub children: Children,
}
#[function_component(UnderlineText)]
pub fn underline_text(props: &UnderlineTextProps) -> Html {
    let children = props.children.clone();

    html! {
        <p class="text-lg underline text-decoration-black decoration-double">{ children }</p>
    }
}
