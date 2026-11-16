// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    // Capture the Dublin Core namespace declaration on <metadata>
    #[serde(rename = "@xmlns:dc", skip_serializing_if = "Option::is_none")]
    pub xmlns_dc: Option<String>,

    // On read: matches <dc:identifier> via alias = "identifier"
    // On write: outputs <dc:identifier> via rename = "dc:identifier"
    #[serde(rename = "dc:identifier", alias = "identifier")]
    pub identifiers: Vec<DcElement>,

    #[serde(rename = "dc:title", alias = "title")]
    pub titles: Vec<DcElement>,

    #[serde(rename = "dc:language", alias = "language")]
    pub languages: Vec<String>,

    #[serde(rename = "dc:creator", alias = "creator", default)]
    pub creators: Vec<DcElement>,

    // EPUB 3 uses <meta property="dcterms:modified">...</meta>
    #[serde(rename = "meta", default)]
    pub metas: Vec<Meta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcElement {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "@property", skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,

    #[serde(rename = "@refines", skip_serializing_if = "Option::is_none")]
    pub refines: Option<String>,

    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text", default)]
    pub value: String,
}
