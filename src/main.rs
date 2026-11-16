// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

use std::env;

mod config;
use config::Config;

mod words;
mod epub;

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
    let mut dict = match words::load_default() {
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

        let extra_words = match words::load(extra_words_filename) {
            Ok(words_map) => {
                dict.extend(words_map);
            }
            _ => {
                eprintln!("Failed to load words from: {}", extra_words_filename);
            std::process::exit(1);
            }
        };
    }
    //println!("{:#?}", );
    use zip;
    use zip::unstable::read::ZipIterable;


    let file = std::fs::File::open(&cfg.input_filename)?;
    let mut archive = ZipIterable::try_new(file, zip::read::Config::default())?;

    for entry in archive.files()? {
        let entry = entry?;
        //println!("{:>20}  {}", entry.size(), entry.name()?);
    }

    //let archive = epub::Reader::new(&cfg.input_filename);

    let (opf_path, mut package) 
        = epub::container::read_opf_from_epub(&cfg.input_filename)?;

    println!("Found OPF at: {}", opf_path);
    println!("Title: {:?}", package.metadata.titles.first());


    Ok(())
   
}
