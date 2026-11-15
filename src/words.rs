// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

use serde::Deserialize;

use std::collections::HashMap;

use std::fs;
use std::io::ErrorKind;

use pub_fields::pub_fields;

const BASE_WORDS: &[u8] = include_bytes!("../data/readlex/readlex_converter.json");

#[derive(Debug, Clone)]
#[derive(Deserialize)]
#[pub_fields]
pub struct Word {
    #[serde(rename = "Shaw")]
    shaw: String,
    tag: String,
}
pub type WordMap = HashMap<String, Vec<Word>>;

pub fn load_default() -> Result<WordMap, Box<dyn std::error::Error>> {

    let json_data = std::str::from_utf8(BASE_WORDS)
                    .expect("Loading default words");

    let words: WordMap = serde_json::from_str(json_data)?;
    Ok(words)
}

pub fn load(json_filename: &String) -> Result<WordMap, Box<dyn std::error::Error>> {

    let json_data = match fs::read_to_string(json_filename) {
        Ok(text) => text,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                eprintln!("File not found: {}", json_filename);
                return Err(Box::new(error));
            }
            ErrorKind::PermissionDenied => {
                eprintln!("Permission denied: {}", json_filename);
                return Err(Box::new(error));
            }
            other_error => {
                eprintln!("Problem reading file: {:?}: {}", other_error, json_filename);
                return Err(Box::new(error));
            }
        },
    };

    let words: WordMap = serde_json::from_str(&json_data)?;
    Ok(words)
}
