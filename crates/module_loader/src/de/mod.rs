use crate::*;
use anyhow::Result;
use serde::{de::DeserializeOwned, *};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{read_dir, read_to_string},
    io::ErrorKind,
    path::Path,
};

#[derive(Debug, Clone, Deserialize)]
struct ModuleManifest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct BookManifest {
    pub chapters: Vec<ChapterManifest>,
}

#[derive(Debug, Clone, Deserialize)]
struct ChapterManifest {
    pub title: String,
    pub allowed_blocks: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct BlockGroupManifest {
    pub color: String,
    pub blocks: HashMap<String, BlockManifest>,
}

#[derive(Debug, Clone, Deserialize)]
struct BlockManifest {
    lable: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    constants: HashMap<String, ConstantManifest>,
}

#[derive(Debug, Clone, Deserialize)]
struct ConstantManifest {
    min: Option<String>,
    max: Option<String>,
}

impl ModuleManifest {
    fn from_mod_path(mod_path: impl AsRef<Path>) -> Result<Self> {
        let manifest_path = mod_path.as_ref().join("module.toml");
        let manifest = read_to_string(manifest_path)?;
        Ok(toml::from_str(&manifest)?)
    }
}

impl BookManifest {
    fn from_mod_path(
        mod_path: impl AsRef<Path>,
    ) -> Result<impl Iterator<Item = Result<(String, Self)>>> {
        iter_manifests_from_folder(mod_path.as_ref().join("books"))
    }
}

impl BlockGroupManifest {
    fn from_mod_path(
        mod_path: impl AsRef<Path>,
    ) -> Result<impl Iterator<Item = Result<(String, Self)>>> {
        iter_manifests_from_folder(mod_path.as_ref().join("blocks"))
    }
}

fn iter_manifests_from_folder<M: DeserializeOwned>(
    folder_path: impl AsRef<Path>,
) -> Result<impl Iterator<Item = Result<(String, M)>>> {
    let iter = read_dir(folder_path)?.map(|entry| {
        let path = entry?.path();

        if path.is_file() && path.extension() == Some(OsStr::new(".toml")) {
            if let Some(Some(name)) = path.file_stem().map(|s| s.to_str()) {
                let src = read_to_string(&path)?;
                let manifest = toml::from_str(&src)?;
                return Ok((name.into(), manifest));
            }
        }

        Err(std::io::Error::new(
            ErrorKind::Other,
            format!("Unexpected item {}", path.display()),
        )
        .into())
    });
    Ok(iter)
}

impl Chapter {
    fn from_manifest(module: &Module, manifest: ChapterManifest) -> Result<Self> {
        Ok(Chapter {
            title: manifest.title,
            allowed_blocks: manifest
                .allowed_blocks
                .iter()
                .map(|name| module.get_block_id(name))
                .collect::<Result<_>>()?,
            blocks: vec![],
        })
    }
}

impl Book {
    fn from_manifest(module: &Module, title: String, manifest: BookManifest) -> Result<Self> {
        Ok(Book {
            title,
            chapters: manifest
                .chapters
                .into_iter()
                .map(|chap_man| Chapter::from_manifest(module, chap_man))
                .collect::<Result<_>>()?,
        })
    }
}

impl BlockTemplate {
    fn from_manifest(
        group_name: impl Into<String>,
        group_color: impl Into<String>,
        block_name: String,
        block: BlockManifest,
    ) -> Result<Self> {
        Ok(BlockTemplate {
            name: block_name,
            group: group_name.into(),
            lable: block.lable,
            inputs: vec![],  // block.inputs,
            outputs: vec![], // block.outputs,
            logic: None,
        })
    }
}

impl Module {
    fn get_block_id(&self, name: &str) -> Result<BlockTemplateId> {
        for (idx, block) in self.blocks.iter().enumerate() {
            if block.name == name {
                return Ok(BlockTemplateId(idx));
            }
        }
        Err(ModError::from("fgr").into())
    }

    fn from_manifest(manifest: ModuleManifest) -> Self {
        Module {
            name: manifest.name,
            description: manifest.description,
            author: manifest.author,
            books: vec![],
            blocks: vec![],
        }
    }

    pub fn from_folder(mod_path: impl AsRef<Path>) -> Result<Self> {
        let mod_manifest = ModuleManifest::from_mod_path(&mod_path)?;
        let mut module = Module::from_manifest(mod_manifest);

        for group_manifest in BlockGroupManifest::from_mod_path(&mod_path)? {
            let (group_name, group) = group_manifest?;

            for (block_name, block_man) in group.blocks {
                let block =
                    BlockTemplate::from_manifest(&group_name, &group.color, block_name, block_man)?;
                module.blocks.push(block);
            }
        }

        for book_manifest in BookManifest::from_mod_path(&mod_path)? {
            let (title, book_manifest) = book_manifest?;
            let book = Book::from_manifest(&module, title, book_manifest)?;
            module.books.push(book);
        }

        Ok(module)
    }
}
