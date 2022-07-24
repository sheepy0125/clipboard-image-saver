/*
 * A shared settings struct
 * Created on 2022-07-22
 */

/*&**** Setup *****/
#![allow(dead_code, unused_imports)]
/* Imports */
use serde::{Deserialize, Serialize};
use std::string::ToString;
use strum_macros::{Display as EnumDisplay, EnumIter, EnumString};

/***** Settings struct *****/
#[derive(EnumIter, EnumString, EnumDisplay, PartialEq, Clone, Serialize, Deserialize)]
pub enum SaveFormat {
    PNG,
    JPG,
    BMP,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub anti_aliasing: bool,
    pub save_path: String,
    pub save_format: SaveFormat,
    pub auto_paste: bool,
}

/***** Functions *****/
/// A getter for default settings
pub fn default_settings() -> Settings {
    Settings {
        anti_aliasing: true,
        save_path: format!("/image.png"),
        save_format: SaveFormat::PNG,
        auto_paste: false,
    }
}

/// Parse a settings JSON file into a Settings struct
/// If parsing failed, this will return an error message as a String
/// Otherwise, it'll return a Settings struct
pub fn parse_settings(settings_text: String) -> Result<Settings, String> {
    let parsed = match serde_json::from_str(&settings_text.as_str()) {
        Ok(parsed) => parsed,
        Err(e) => return Err(format!("Failed to parse the settings text: {}", e)),
    };

    Ok(parsed)
}
