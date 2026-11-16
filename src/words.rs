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

/// A Word is an instance of a Shavian word in data/readlex/readlex_converter.json.
pub struct Word {
    #[serde(rename = "Shaw")]
    /// The Shavian alphabet representation of the word.
    /// This field may contain multiple words separated by '⬌'.
    /// In those cases, the words are heteronyms, where words are spelt the
    /// same in the Latin alphabet but have different pronunciations.
    shaw: String,
    /// The part of speech for this word:
    /// 0 (all), DT, FW, IN, JJ, JJR, MD, NN, NNP, NNS, PRP, 
    /// RB, SYM, UH, VB, VBD, VBG, VBN, VBP, VBZ, WP.
    /// The tags are according to <http://www.natcorp.ox.ac.uk/docs/gramtag.html>.
    tag: String,
}

/// WordMap matches the structure of data/readlex/readlex_converter.json.
/// The keys of the map are latin alphat words.
/// Each value is a vector of possible Shavian transliterations.
pub type WordMap = HashMap<String, Vec<Word>>;

/// Return the default Read Lexicon dictionary.
pub fn load_default() -> Result<WordMap, Box<dyn std::error::Error>> {

    let json_data = std::str::from_utf8(BASE_WORDS)
                    .expect("Loading default words");

    let words: WordMap = serde_json::from_str(json_data)?;
    Ok(words)
}

/// Reaturn a dictionary from a given filename. 
/// The filename must use the same format as the Read Lexicon dictionary.
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
