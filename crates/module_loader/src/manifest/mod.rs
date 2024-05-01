mod from_manifest;
mod from_path;

use crate::*;
use serde::{de::DeserializeOwned, *};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{read_dir, read_to_string},
};

pub use from_manifest::*;
pub use from_path::*;

#[derive(Debug, Clone)]
pub struct ModuleManifestBundle {
    pub module: ModuleManifest,
    pub books: HashMap<String, BookManifest>,
    pub blocks: HashMap<String, BlockGroupManifest>,
    pub scenes: ScenesManifest,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModuleManifest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: Vec<String>,

    /// Modules that need to be completed to start this module.
    ///
    /// This module will only be able to use external blocks of
    /// the modules that requires.
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookManifest {
    /// Books from the same module that need to be completed to start this book.
    pub requirements: Vec<String>,
    /// Blocks or Block Groups that are allowed to use in any chapter of the book.
    pub allowed_blocks: Vec<String>,
    pub chapters: Vec<ChapterManifest>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChapterManifest {
    pub title: String,
    /// Blocks or Block Groups that are allowed to use only in this chapter.
    pub allowed_blocks: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlockGroupManifest {
    /// The color that the defined blocks of the group will have.
    pub color: String,
    pub blocks: HashMap<String, BlockManifestRef>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum BlockManifestRef {
    Ref(String),
    Defined(BlockManifest),
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlockManifest {
    lable: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScenesManifest {
    scenes: HashMap<String, SceneManifest>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SceneManifest {
    inputs: Vec<String>,
    outputs: Vec<String>,
}
