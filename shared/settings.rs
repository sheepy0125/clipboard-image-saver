/*
 * A shared settings struct
 * Created on 2022-07-22
 */

/***** Setup *****/
#![allow(dead_code, unused_imports, clippy::derive_partial_eq_without_eq)]
/* Imports */
use serde::{Deserialize, Serialize};
use std::string::ToString;
use strum_macros::{Display as EnumDisplay, EnumIter, EnumString};

/***** Settings struct *****/
#[derive(EnumIter, EnumString, EnumDisplay, PartialEq, Clone, Serialize, Deserialize)]
pub enum SaveFormat {
    Png,
    Jpg,
    Bmp,
    Ico,
    Tiff,
    Gif,
    Tga,
}
impl Default for SaveFormat {
    /// Default save format
    fn default() -> SaveFormat {
        SaveFormat::Png
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub anti_aliasing: bool,
    pub save_path: String,
    pub save_format: SaveFormat,
    pub auto_paste: bool,
    pub zoom_by: i32,
}
impl Default for Settings {
    /// Default settings
    fn default() -> Settings {
        Settings {
            anti_aliasing: true,
            save_path: "./image".to_string(),
            save_format: SaveFormat::default(),
            auto_paste: false,
            zoom_by: 10,
        }
    }
}
impl Settings {
    /// Parse a settings JSON file into a Settings struct
    pub fn parse(settings_text: String) -> Result<Settings, String> {
        let parsed = match serde_json::from_str(settings_text.as_str()) {
            Ok(parsed) => parsed,
            Err(e) => return Err(format!("Failed to parse the settings text: {}", e)),
        };
        Ok(parsed)
    }
}
