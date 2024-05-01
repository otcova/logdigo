//! functions that loads manifests from files

use super::*;
use std::path::Path;

impl ModuleManifestBundle {
    pub fn from_mod_path(mod_path: impl AsRef<Path>) -> Self {
        Self {
            module: ModuleManifest::from_mod_path(&mod_path),
            blocks: BlockGroupManifest::from_mod_path(&mod_path).collect(),
            books: BookManifest::from_mod_path(&mod_path).collect(),
            scenes: ScenesManifest::from_mod_path(&mod_path),
        }
    }
}

impl ModuleManifest {
    fn from_mod_path(mod_path: impl AsRef<Path>) -> Self {
        let manifest_path = mod_path.as_ref().join("module.toml");
        let manifest = read_to_string(manifest_path).unwrap();
        toml::from_str(&manifest).unwrap()
    }
}

impl ScenesManifest {
    fn from_mod_path(mod_path: impl AsRef<Path>) -> Self {
        let manifest_path = mod_path.as_ref().join("scenes.toml");
        let manifest = read_to_string(manifest_path).unwrap();
        toml::from_str(&manifest).unwrap()
    }
}

impl BookManifest {
    fn from_mod_path(mod_path: impl AsRef<Path>) -> impl Iterator<Item = (String, Self)> {
        iter_manifests_from_folder(mod_path.as_ref().join("books"))
    }
}

impl BlockGroupManifest {
    fn from_mod_path(mod_path: impl AsRef<Path>) -> impl Iterator<Item = (String, Self)> {
        iter_manifests_from_folder(mod_path.as_ref().join("blocks"))
    }
}

fn iter_manifests_from_folder<M: DeserializeOwned>(
    folder_path: impl AsRef<Path>,
) -> impl Iterator<Item = (String, M)> {
    let iter = read_dir(folder_path).unwrap().map(|entry| {
        let path = entry.unwrap().path();

        if path.is_file() && path.extension() == Some(OsStr::new(".toml")) {
            if let Some(Some(name)) = path.file_stem().map(|s| s.to_str()) {
                let src = read_to_string(&path).unwrap();
                let manifest = toml::from_str(&src).unwrap();
                return (name.into(), manifest);
            }
        }

        panic!("Unexpected item {}", path.display());
    });
    (iter)
}
