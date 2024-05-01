//! functions that construct modules from manifests

use std::sync::Arc;

use crate::*;

use super::BlockManifestRef;

impl BlockDescSubset {
    fn from_manifest(blocks: &ModuleBlocks, subset: &str) -> Self {
        if let Some(block_desc) = blocks.get(subset).cloned() {
            return Self { block_desc };
        }
        todo!()
    }
}

impl Chapter {
    fn from_manifest(blocks: &ModuleBlocks, book_id: BookId, manifest: ChapterManifest) -> Self {
        (Chapter {
            id: ChapterId {
                book_id,
                title: manifest.title,
            },
            allowed_blocks: manifest
                .allowed_blocks
                .iter()
                .map(|desc_subset| BlockDescSubset::from_manifest(blocks, desc_subset))
                .collect(),
            completion_status: ChapterCompletionStatus::NotStarted,
        })
    }
}

impl Book {
    fn from_manifest(blocks: &ModuleBlocks, id: BookId, manifest: BookManifest) -> Self {
        (Book {
            chapters: manifest
                .chapters
                .into_iter()
                .map(|chapter_man| Chapter::from_manifest(blocks, id.clone(), chapter_man))
                .collect(),
            id,
        })
    }
}

impl BlockDesc {
    fn from_manifest(
        group_name: String,
        group_color: &str,
        block_name: String,
        block: BlockManifest,
    ) -> Self {
        (BlockDesc {
            id: BlockDescId { name: block_name },
            color: group_color.into(),
            group: group_name,
            lable: block.lable,
            inputs: vec![],  // block.inputs,
            outputs: vec![], // block.outputs,
            logic: None,
        })
    }
}

impl BlockManifestRef {
    /// Returns the block manifest loading it if necesary
    fn manifest(self) -> BlockManifest {
        match self {
            Self::Ref(id) => todo!(),
            Self::Defined(manifest) => manifest,
        }
    }
}

impl Module {
    fn from_manifest(manifest: ModuleManifestBundle, namespace: String) -> Self {
        let module_id = ModuleId {
            name: manifest.module.name,
            namespace,
        };
        let blocks = manifest
            .blocks
            .into_iter()
            .flat_map(|(group_name, group)| {
                group.blocks.into_iter().map(move |(block_name, block)| {
                    let manifest = block.manifest();
                    let block_desc = BlockDesc::from_manifest(
                        group_name.clone(),
                        &group.color,
                        block_name.clone(),
                        manifest,
                    );
                    (block_name, Arc::new(block_desc))
                })
            })
            .collect();
        let books = manifest
            .books
            .into_iter()
            .map(|(title, book_man)| {
                let book_id = BookId {
                    module_id: module_id.clone(),
                    title: title.clone(),
                };
                let book = Book::from_manifest(&blocks, book_id, book_man);
                (title, book)
            })
            .collect();

        Module {
            id: module_id,
            description: manifest.module.description,
            author: manifest.module.author,
            books,
            blocks,
        }
    }
}
