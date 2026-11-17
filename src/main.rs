// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

use std::env;

mod config;
use config::Config;

use config::words;
mod epub;
mod shaw;
mod transliterator;
use transliterator::Transliterator;

fn main()  -> Result<(), Box<dyn std::error::Error>>  {

    // Collect arguments into a vector
    let args: Vec<String> = env::args().collect();

    // args[0] is always the path to the running binary
    let program_name = &args[0];

    // Check if enough user arguments were passed
    if args.len() != 2 {
        eprintln!("Usage: {} config_file_path", program_name);
        std::process::exit(1);
    }

    let config_filename = &args[1];


    println!("config_filename = {}", config_filename);
    let cfg = match Config::load(config_filename) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Reading config file failed!");
            //return Err(Box::new(error));
            std::process::exit(1);
        } 
    };
    //println!("{:#?}", cfg);
    let mut dict = match config::words::load_default() {
        Ok(words_map) => words_map,
        _ => {
            eprintln!("Failed to load words from default dictionary.");
            std::process::exit(1);
        }
    };
    let extra_words_filenames = match cfg.words_filenames {
        Some(value) => value,
        _ => Vec::<String>::new(),
    };
    for extra_words_filename in &extra_words_filenames {
        println!("{:#?}", extra_words_filename);

        let extra_words = match config::words::load(extra_words_filename) {
            Ok(words_map) => {
                dict.extend(words_map);
            }
            _ => {
                eprintln!("Failed to load words from: {}", extra_words_filename);
            std::process::exit(1);
            }
        };
    }

    let epub_reader = epub::Reader::new(&cfg.input_filename)?;
    let epub_package = epub_reader.get_package();

    let trans = Transliterator::new();

    let items = epub_package.manifest.items;
    for item in items {
        //println!("{:#?}", item);
        if item.media_type == "application/xhtml+xml" {
            let Some(bytes) = epub_reader.get_data(item.id) else {
               
                eprintln!("Failed to load content from: {}", item.href);
                std::process::exit(1);
            };
            let content = String::from_utf8(bytes.clone())?;
            //println!("{}", content);
            let text = shaw::html2shaw(&content, trans);
            println!("{}", text);
            break;
        }
    }


    Ok(())
   
}
