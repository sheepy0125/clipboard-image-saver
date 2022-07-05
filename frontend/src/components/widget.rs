/*
 * Widget component
 * Created on 2022-07-05
 */

/***** Setup *****/
/* Imports */
use yew::prelude::*;

/***** Widget component *****/
#[derive(PartialEq, Properties)]
pub struct WidgetProps {
    pub children: Children,
    pub class: Option<String>,
}
#[function_component(Widget)]
pub fn widget(props: &WidgetProps) -> Html {
    let extra_class = props.class.clone().unwrap_or_default();
    html! {
        <div class={
            format!(
                "p-2 lg:p-4 bg-widget border-2 bg-gray-700 bg-opacity-80 border-pink-200 text-blue-200 rounded-lg shadow-lg {}",
                extra_class
            )
        }>
            { props.children.clone() }
        </div>
    }
}
