/*
 * Clipboard image saver
 * Created on 2022-07-02
 */

/***** Setup *****/
#![allow(clippy::derive_partial_eq_without_eq)]
/* Imports */
#[path = "./components/main_view.rs"]
mod main_view;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/***** Main *****/
fn main() {
    yew::start_app::<main_view::MainView>();
}
