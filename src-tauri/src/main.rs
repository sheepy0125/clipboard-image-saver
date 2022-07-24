/*
 * Image clipboard saver Tauri backend
 * Created on 2022-07-02
 */

/***** Setup *****/
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(const_io_structs)]

/* Imports */
extern crate base64;
use arboard::Clipboard;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat, RgbaImage};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, write},
    io::Cursor,
};
#[path = "../../shared/src/settings.rs"]
mod settings;

/***** Global image cursor *****/
/* Yes, I'm aware that using a global isn't a good idea in most cases.
 * But a global seems useful for this case.
 * Reasoning: When the user wants to save the image, don't fetch it from the clipboard
 * again if it's already been fetched once. */
static mut CLIPBOARD_PNG_IMAGE_CURSOR: Cursor<Vec<u8>> = Cursor::new(Vec::new());

/***** Commands *****/
/// Read the clipboard image information.
/// If the clipboard read was successful and was an image, this will return
/// a base64 encoded String with the image data (to be used in a data URL).
/// If not, it will return an error message as a String.
#[tauri::command]
fn read_clipboard() -> Result<String, String> {
    // Get raw image data from clipboard
    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => clipboard,
        Err(e) => return Err(format!("Failed to get clipboard handler: {}", e)),
    };
    let image_data = match clipboard.get_image() {
        Ok(image_data) => image_data,
        Err(e) => return Err(format!("Failed to get an image from the clipboard: {}", e)),
    };

    // Convert the raw image data (bytes is image::ImageBuffer.into_raw())
    let image_buf: RgbaImage = match ImageBuffer::from_raw(
        image_data.width as u32,
        image_data.height as u32,
        image_data.bytes.into_owned(),
    ) {
        Some(buf) => buf,
        None => return Err("Failed to convert the raw bytes into an image buffer".to_string()),
    };

    // Write the PNG encoded image data into a vector
    // The vector must be wrapped in a cursor so it satisfies `Seek` and `Read` traits
    let encoded_buf: Vec<u8>;
    let image = DynamicImage::ImageRgba8(image_buf);
    unsafe {
        CLIPBOARD_PNG_IMAGE_CURSOR.set_position(0);
        image
            .write_to(&mut CLIPBOARD_PNG_IMAGE_CURSOR, ImageOutputFormat::Png) // TODO: different formats
            .unwrap();
        encoded_buf = CLIPBOARD_PNG_IMAGE_CURSOR.clone().into_inner();
    }
    // Convert to base64 to be used as a data URL (e.g. `data:png;base64,{base64_encoded}`)
    let base64_encoded = base64::encode(encoded_buf);

    Ok(base64_encoded)
}

/// Save the image to a file
/// If the save was successful, this command will return a Ok(()).
/// Otherwise, it'll return the error as a String.
#[tauri::command]
fn save_image(path: String) -> Result<(), String> {
    /* TODO: different file formats, right now it's PNG */

    // Get PNG image buffer
    let buf: Vec<u8>;
    unsafe {
        buf = CLIPBOARD_PNG_IMAGE_CURSOR.clone().into_inner();
    }

    // Write to file
    match write(&path, buf) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(format!("Failed to save image to {}: {}", path, e)),
    }
}

/// Read the settings file
/// If the parsing was unsuccessful, this will return an error message as a String
/// Otherwise, it'll return the JSON information of the settings as a String
#[tauri::command]
fn read_settings() -> Result<String, String> {
    let file_text = match read_to_string("../settings.json") {
        Ok(file_text) => file_text,
        Err(e) => return Err(format!("Failed to load settings file: {}", e)),
    };

    Ok(file_text)
}

/// Save settings
/// If the saving was unsuccessful, this will return an error message as a String
/// Otherwise, it'll return `()`
#[tauri::command]
fn save_settings(settings: String) -> Result<(), String> {
    match write("../settings.json", settings) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(format!("Failed to save settings: {}", e)),
    }
}

/***** Main *****/
fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(if cfg!(target_os = "macos") {
            tauri::Menu::os_default(&context.package_info().name)
        } else {
            tauri::Menu::default()
        })
        .invoke_handler(tauri::generate_handler![
            read_clipboard,
            save_image,
            save_settings,
            read_settings,
        ])
        .run(context)
        .expect("error while running tauri application");
}
