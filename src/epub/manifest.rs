// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT


use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(rename = "item", default)]
    pub items: Vec<ManifestItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManifestItem {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@href")]
    pub href: String,

    #[serde(rename = "@media-type")]
    pub media_type: String,

    #[serde(rename = "@properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
}
