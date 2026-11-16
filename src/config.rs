// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

use serde::Deserialize;

use std::fs;
use std::io::ErrorKind;
use std::path::Path;


use pub_fields::pub_fields;

const XML_LANG: &str = "en-Shaw"; // sets xml-lang and dc:language

#[derive(Debug, Clone)]
#[derive(Deserialize)]
#[pub_fields]
pub struct Config {
    input_filename: String, // relative to config file
    output_dir: String,  // relative to config file

    // default to "-shavian"
    #[serde(default = "default_output_basename_suffix")]
    output_basename_suffix: String,
    
    words_filenames: Option<Vec<String>>,  // relative to config file
    phrases_filenames: Option<Vec<String>>,  // relative to config file
    output_unknown_words_file: Option<String>,  // relative to config file

    // All files under this directory will be copied into the epub
    // Any existing images with the same name in the epub will be overwritten 
    copy_data: Option<String>,  // relative to config file

    // List of items to omit
    remove_items: Option<Vec<String>>,
    no_transliteration: Option<Vec<String>>,

    metadata: Metadata,
}

fn default_output_basename_suffix() -> String {
    return "-shavian".to_string();
}

#[derive(Debug, Clone)]
#[derive(Deserialize)]
pub struct Metadata {
    // Set <dc:date>
    dc_date: Option<String>,

    // replace the dc:title element in the metadata,
    dc_title: Option<String>,

    // Replaces <dc:identifier> content, or added if it doesn't already existing
    dc_identifier: Option<String>,

}

impl Config {

    fn make_relative_file(file1: &String, file2: &String) -> String {

        let file1_path = Path::new(file1);
        let file2_path = Path::new(file2);

        if file2_path.is_relative() {
            let parent = file1_path.parent().unwrap();
            let temp = &parent.join(file2_path);
            return temp.to_str().unwrap().to_string();
        }
        return file2.clone();
    }

    pub fn load(config_filename: &String) -> Result<Self, Box<dyn std::error::Error>> {

        let toml_data = match fs::read_to_string(config_filename) {
            Ok(text) => text,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    eprintln!("File not found: {}", config_filename);
                    return Err(Box::new(error));
                }
                ErrorKind::PermissionDenied => {
                    eprintln!("Permission denied: {}", config_filename);
                    return Err(Box::new(error));
                }
                other_error => {
                    eprintln!("Problem reading file: {:?}: {}", other_error, config_filename);
                    return Err(Box::new(error));
                }
            },
        };

        let config: Config = toml::from_str(&toml_data)?;
        let mut result = config.clone();
        /* expand the paths in config, relative to config_filename
            input_filename: String
            output_dir: String
            words_filenames: Option<Vec<String>>
            phrases_filenames: Option<Vec<String>>
            output_unknown_words_file: Option<String>
            copy_data: Option<String>
        */
        result.input_filename = Self::make_relative_file(
            &config_filename, &config.input_filename
        );
        result.output_dir = Self::make_relative_file(
            &config_filename, &config.output_dir
        );
        match config.words_filenames {
            Some(items) => {

                let mut filenames = Vec::<String>::new();
                for item in items {
                    filenames.push(Self::make_relative_file(
                    &config_filename, &item));
                }
            result.words_filenames = Some(filenames);

            },
            _ => {},
        };
        match config.phrases_filenames {
            Some(items) => {

                let mut filenames = Vec::<String>::new();
                for item in items {
                    filenames.push(Self::make_relative_file(
                    &config_filename, &item));
                }
            result.phrases_filenames = Some(filenames);

            },
            _ => {},
        };
        match config.output_unknown_words_file {
            Some(item) => {

                result.output_unknown_words_file = Some(
                    Self::make_relative_file(
                        &config_filename, &item));
                }
            _ => {},
        };
        match config.copy_data {
            Some(item) => {

                result.copy_data = Some(
                    Self::make_relative_file(
                        &config_filename, &item));
                }
            _ => {},
        };
        //println!("{:#?}", result);

        Ok(result)
    }

}

