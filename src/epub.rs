// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

pub mod reader;

pub use crate::epub::reader::Reader;

pub mod container;

mod metadata;
//pub use crate::epub::metadata::Metadata;

mod manifest;
//pub use crate::epub::manifest::Manifest;

pub mod package;
pub use crate::epub::package::Package;

mod spine;
//pub use crate::epub::spine::Spine;

