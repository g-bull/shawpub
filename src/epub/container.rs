// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

use quick_xml::de::from_str;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

use super::Package;

// --- 1. Structs for META-INF/container.xml ---

#[derive(Debug, Deserialize)]
#[serde(rename = "container")]
struct Container {
    rootfiles: Rootfiles,
}

#[derive(Debug, Deserialize)]
struct Rootfiles {
    #[serde(rename = "rootfile")]
    rootfile: Vec<Rootfile>,
}

#[derive(Debug, Deserialize)]
struct Rootfile {
    #[serde(rename = "@full-path")]
    full_path: String,

    #[serde(rename = "@media-type")]
    media_type: String,
}

// --- 2. Function to locate and parse the OPF from an EPUB ---

pub fn read_opf_from_epub<P: AsRef<Path>>(
    epub_path: P,
) -> Result<(String, Package), Box<dyn std::error::Error>> {
    let file = File::open(epub_path)?;
    let mut archive = ZipArchive::new(file)?;

    // Step A: Read META-INF/container.xml to find the .opf path
    let container_xml = {
        let mut container_file = archive.by_name("META-INF/container.xml")?;
        let mut contents = String::new();
        container_file.read_to_string(&mut contents)?;
        contents
    };

    let container: Container = from_str(&container_xml)?;
    let opf_path = container
        .rootfiles
        .rootfile
        .into_iter()
        .find(|rf| rf.media_type == "application/oebps-package+xml")
        .map(|rf| rf.full_path)
        .ok_or("No OEBPS package rootfile found in container.xml")?;

    // Step B: Read and parse the .opf file using the discovered path
    let opf_xml = {
        let mut opf_file = archive.by_name(&opf_path)?;
        let mut contents = String::new();
        opf_file.read_to_string(&mut contents)?;
        contents
    };

    let package: Package = from_str(&opf_xml)?;

    // Return both the internal path (useful when saving back) and the parsed struct
    Ok((opf_path, package))
}
/* 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (opf_path, mut package) = read_opf_from_epub("book.epub")?;

    println!("Found OPF at: {}", opf_path);
    println!("Title: {:?}", package.metadata.titles.first());

    // Modify package here...
    if let Some(title) = package.metadata.titles.first_mut() {
        title.value = "New Title".to_string();
    }

    Ok(())
}
    */
