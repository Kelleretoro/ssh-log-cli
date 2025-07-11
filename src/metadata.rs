use std::io::Read;

use base64::DecodeError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PTYMetadata {
    pub term: Option<String>,
    pub width: u32,
    pub height: u32,
    pub modes: Vec<(String, u32)>,
}
