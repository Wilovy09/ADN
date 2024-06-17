use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::str::FromStr;

use thiserror::Error;
use toml_edit::Item;
use toml_edit::{DocumentMut, Table};

#[derive(Debug, Error)]
pub enum LspConfigError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Toml(#[from] toml_edit::TomlError),
}

pub fn add_lsp_config(read_file: &Path, content_lsp: &str) -> Result<String, LspConfigError> {
    let content = File::open(read_file)
        .and_then(|mut file| {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            Ok(content)
        })
        .unwrap_or_else(|_| String::new());
    // TODO: handle errors
    let mut languages_toml = DocumentMut::from_str(&content)?;
    let lsp_config = DocumentMut::from_str(content_lsp)?;
    merge_toml(&mut languages_toml, &lsp_config);

    let content = languages_toml.to_string();
    std::fs::write(read_file, &content)?;

    Ok(content)
}

fn merge_toml(languages_toml: &mut Table, lsp_config: &Table) {
    for (key, item) in lsp_config {
        match (item, languages_toml.get_mut(key)) {
            (Item::None, _) => continue, // if lsp config is nothing continue
            (item, None | Some(Item::None)) => {
                // if languages.toml doesnt have the key, add it
                languages_toml.insert(key, item.clone());
                println!("inserted config")
            }
            (Item::Table(lsp_config), Some(Item::Table(languages_toml))) => {
                // recursively merge the tables
                merge_toml(languages_toml, lsp_config);
            }
            _ => continue, // if languages.toml has the key, dont insert config
        }
    }
}
