// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Spine {
    #[serde(rename = "@toc", skip_serializing_if = "Option::is_none")]
    pub toc: Option<String>,

    #[serde(rename = "itemref", default)]
    pub itemrefs: Vec<ItemRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemRef {
    #[serde(rename = "@idref")]
    pub idref: String,

    #[serde(rename = "@linear", skip_serializing_if = "Option::is_none")]
    pub linear: Option<String>,
}
