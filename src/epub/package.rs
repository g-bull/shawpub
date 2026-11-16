// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

use quick_xml::de::from_str;
use quick_xml::se::Serializer;
use serde::{Deserialize, Serialize};

use super::manifest::Manifest;
use super::metadata::Metadata;
use super::spine::Spine;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "package")]
pub struct Package {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,

    #[serde(rename = "@version")]
    pub version: String,

    #[serde(rename = "@unique-identifier")]
    pub unique_identifier: String,

    #[serde(rename = "@xml:lang", alias = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,

    pub metadata: Metadata,
    pub manifest: Manifest,
    pub spine: Spine,
}


/*
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_opf = r#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://idpf.org" version="3.0" unique-identifier="pub-id" xml:lang="en">
  <metadata xmlns:dc="http://purl.org">
    <dc:identifier id="pub-id">urn:uuid:12345678-1234-1234-1234-123456789abc</dc:identifier>
    <dc:title id="t1">Original Book Title</dc:title>
    <dc:language>en</dc:language>
    <dc:creator id="c1">Jane Doe</dc:creator>
    <meta property="dcterms:modified">2025-01-01T12:00:00Z</meta>
  </metadata>
  <manifest>
    <item id="nav" href="nav.xhtml" media-type="application/xhtml+xml" properties="nav"/>
    <item id="ch1" href="chapter1.xhtml" media-type="application/xhtml+xml"/>
  </manifest>
  <spine>
    <itemref idref="ch1"/>
  </spine>
</package>"#;

    // 1. Deserialize XML into Rust structs
    let mut package: Package = from_str(sample_opf)?;

    // 2. Modify fields
    if let Some(title) = package.metadata.titles.first_mut() {
        title.value = "Updated Book Title (2nd Edition)".to_string();
    }

    if let Some(modified) = package
        .metadata
        .metas
        .iter_mut()
        .find(|m| m.property.as_deref() == Some("dcterms:modified"))
    {
        modified.value = "2026-11-16T05:00:00Z".to_string();
    }

    // Add a new CSS file to the manifest
    package.manifest.items.push(ManifestItem {
        id: "style".to_string(),
        href: "styles.css".to_string(),
        media_type: "text/css".to_string(),
        properties: None,
    });

    // 3. Reserialize back to pretty-printed XML
    let mut xml_output = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    let mut serializer = Serializer::new(&mut xml_output);
    serializer.indent(' ', 2);
    package.serialize(serializer)?;

    println!("{}", xml_output);

    Ok(())
}
*/
