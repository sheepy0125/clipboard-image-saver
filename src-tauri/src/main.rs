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
use image::{DynamicImage, ImageBuffer, ImageFormat, ImageOutputFormat, RgbaImage};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, write},
    io::Cursor,
    path::PathBuf,
    str::FromStr,
    sync::RwLock,
};
use tauri::{
    api::{dialog::blocking::FileDialogBuilder, path::picture_dir},
    State,
};
#[path = "../../shared/src/settings.rs"]
mod settings;

/***** Globals *****/
static SETTINGS_PATH: &str = "../settings.json";

#[derive(Default)]
pub struct ImageDataState(pub RwLock<ImageData>);
#[derive(Default)]
pub struct ImageData {
    /// The clipboard image encoded as a dynamic image
    pub clipboard_dynamic_image: DynamicImage,
    /// The image converted to whatever format the user selected
    /// The vector must be wrapped in a cursor so it satisfies `Seek` and `Read` traits
    pub clipboard_image_cursor: Cursor<Vec<u8>>,
}
impl ImageData {
    /// Convert an image to the format specified
    pub fn convert_encoded_cursor_with_format(
        &mut self,
        format: settings::SaveFormat,
    ) -> Result<(), String> {
        let format = ImageOutputFormat::from(
            ImageFormat::from_extension((format).to_string().to_lowercase()).unwrap(),
        );

        // Clear cursor
        self.clipboard_image_cursor.set_position(0);
        self.clipboard_image_cursor.get_mut().clear();

        // Write into cursor
        match self
            .clipboard_dynamic_image
            .write_to(&mut self.clipboard_image_cursor, format)
        {
            Ok(_) => (),
            Err(e) => {
                return Err(format!(
                    "Failed to write dynamic image from clipboard into buffer: {}",
                    e
                ))
            }
        }

        Ok(())
    }
}

/***** Commands *****/
/// Read the clipboard image information.
#[tauri::command]
fn read_clipboard(state: State<ImageDataState>) -> Result<String, String> {
    let mut state_guard = match state.0.write() {
        Ok(state_guard) => state_guard,
        Err(_) => return Err(format!("Failed to get a state RwLockGuard")),
    };

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

    // Write dynamic image to state
    let dynamic_image = DynamicImage::ImageRgba8(image_buf);
    state_guard.clipboard_dynamic_image = dynamic_image;

    // Get encoded PNG cursor
    match state_guard.convert_encoded_cursor_with_format(settings::SaveFormat::PNG) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    Ok(base64::encode(state_guard.clipboard_image_cursor.get_ref()))
}

/// Get the path to save the image
#[tauri::command]
async fn get_save_path(format: String) -> Result<String, String> {
    let directory = match picture_dir() {
        Some(picture_dir) => picture_dir,
        None => PathBuf::from("."),
    };
    let file = match FileDialogBuilder::new()
        .set_title("Where would you like to save the image?")
        .set_directory(directory)
        .add_filter(format.to_uppercase(), &[format.to_lowercase().as_str()])
        .save_file()
    {
        Some(file) => file.into_os_string().into_string().unwrap(),
        None => return Err("User canceled save".to_string()),
    };

    Ok(file)
}

/// Save the image to a file with a specified format
#[tauri::command]
fn save_image(state: State<ImageDataState>, path: String, format: String) -> Result<(), String> {
    let mut state_guard = match state.0.write() {
        Ok(state_guard) => state_guard,
        Err(_) => return Err(format!("Failed to get a state RwLockGuard")),
    };

    let format = settings::SaveFormat::from_str(format.as_str()).unwrap();

    // Convert the buffer to the save format
    match state_guard.convert_encoded_cursor_with_format(format) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    // Write to file
    match write(&path, state_guard.clipboard_image_cursor.get_ref()) {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to save image to {}: {}", path, e)),
    }

    Ok(())
}

/// Load the settings file and return the text contents of it
#[tauri::command]
fn load_settings() -> Result<String, String> {
    let file_text = match read_to_string(SETTINGS_PATH) {
        Ok(file_text) => file_text,
        Err(e) => return Err(format!("Failed to load settings file: {}", e)),
    };

    Ok(file_text)
}

/// Save settings
#[tauri::command]
fn save_settings(settings: String) -> Result<(), String> {
    match write(SETTINGS_PATH, settings) {
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
        .manage(ImageDataState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            read_clipboard,
            save_image,
            save_settings,
            load_settings,
            get_save_path,
        ])
        .run(context)
        .expect("error while running tauri application");
}
