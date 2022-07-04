/*
 * Clipboard image saver
 * Created on 2022-07-02
 */

/***** Setup ******/
/* Imports */
use yew::prelude::*;
#[path = "./components/main_view.rs"]
mod main_view;

/***** App *****/
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main_view::MainView />
    }
}

/***** Main ******/
fn main() {
    yew::start_app::<App>();
}
