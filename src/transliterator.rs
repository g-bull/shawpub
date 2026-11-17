
// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

pub struct Transliterator {

}

impl Transliterator {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn transliterate(&self, text : String)  -> String {
        text.to_uppercase()
    }
}