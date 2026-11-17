// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

const BASE_PHRASES: &[u8] = include_bytes!("./data/readlex/static/readlex_converter_phrases.json");
// Note: the ReadLex phrases file is actually a simple text file,
// with one phrase per line.
// It can be read using a CSV reader configure for one colmn and no header row.

