use crate::prelude::StdResult;
use anyhow::Result;
use serde::Deserialize;
use std::{cmp::Ordering, num::ParseFloatError};

#[derive(Debug, Deserialize)]
pub struct ResultEntryHeader {
    pub similarity: String,
    pub thumbnail: Option<String>,
    pub index_id: u32,
    pub index_name: Option<String>,
    pub dupes: u32,
    pub hidden: u32,
}

impl ResultEntryHeader {
    pub fn similarity_as_f64(&self) -> StdResult<f64, ParseFloatError> {
        self.similarity.parse()
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct ResultEntryData {
    #[serde(default)]
    pub ext_urls: Vec<String>,
    pub title: Option<String>,
    pub pixiv_id: Option<u32>,
    pub yandere_id: Option<u32>,
    pub danbooru_id: Option<u32>,
    pub gelbooru_id: Option<u32>,
    pub da_id: Option<String>,
    pub tweet_id: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResultEntry {
    pub header: ResultEntryHeader,
    pub data: ResultEntryData,
}

impl ResultEntry {
    /// Checks if saucenao is confident in the result or not
    pub fn is_credible(&self) -> Result<bool> {
        let similarity = self.header.similarity_as_f64()?;

        match similarity.partial_cmp(&89.0) {
            Some(Ordering::Greater) | Some(Ordering::Equal) => Ok(true),
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<ResultEntry>,
}
