/*
 * Main view
 * Created on 2022-07-02
 */

/***** Setup *****/
/* Imports */
#![allow(clippy::duplicate_mod)]
use yew::{prelude::*, ContextProvider};
#[path = "./clipboard_image.rs"]
mod clipboard_image;
#[path = "./controls.rs"]
mod controls;
#[path = "../../../shared/settings.rs"]
pub mod global_settings;
#[path = "./settings.rs"]
mod settings;

/***** Main view *****/
#[function_component(MainView)]
pub fn main_view() -> Html {
    // Settings
    let settings_state = use_state(global_settings::Settings::default);
    let on_update_settings = {
        let settings_state = settings_state.clone();
        Callback::from(move |new_settings: global_settings::Settings| {
            settings_state.set(new_settings)
        })
    };

    // Paste button callback from control widget
    let should_update_clipboard = use_state_eq(|| false);
    let on_should_update_clipboard = {
        let should_update_clipboard = should_update_clipboard.clone();
        Callback::from(move |_| {
            should_update_clipboard.set(true);
            should_update_clipboard.set(false);
        })
    };

    // Right side collapsed
    let right_side_collapsed_state = use_state_eq(|| true);
    let on_collapsed_toggle_click = {
        let right_side_collapsed_state = right_side_collapsed_state.clone();
        Callback::from(move |_| right_side_collapsed_state.set(!*right_side_collapsed_state))
    };

    html! {
        <ContextProvider<global_settings::Settings> context={(*settings_state).clone()}>
            <div class="flex p-2 h-screen">
                // Image view
                <div class="grid flex-1 w-full">
                    <clipboard_image::ClipboardImage should_update_clipboard={ *should_update_clipboard } />
                </div>
                // Settings and controls
                <div class={
                    format!(
                        "ml-2 flex-col flex-initial {}",
                        match *right_side_collapsed_state {
                            false => "hidden",
                            true => "flex"
                        }
                    )
                }>
                    <div class="flex-1 mb-2">
                        <settings::Settings { on_update_settings } />
                    </div>
                    <div class="flex-initial">
                        <controls::Controls { on_should_update_clipboard } />
                    </div>
                </div>
                // Collapse
                <button onclick={ on_collapsed_toggle_click } class="ml-2 text-white">
                    {
                        match *right_side_collapsed_state {
                            true => ">",
                            false => "<"
                        }
                    }
                </button>
            </div>
        </ContextProvider<global_settings::Settings>>
    }
}
