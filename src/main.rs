pub mod languages;
pub mod utilities;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// This parser the args
#[derive(Parser)]
struct Argumentos {
    /// This is for 'languages.toml'
    #[clap(long, short)]
    #[arg(default_value=get_default_config_path().into_os_string())]
    config: Option<PathBuf>,

    /// This is for 'node_modules/'
    #[clap(long, short)]
    #[arg(default_value=get_default_npm_folder().into_os_string())]
    npm_folder: Option<PathBuf>,

    #[clap(subcommand)]
    cmd: Comandos,
}

fn get_default_config_path() -> PathBuf {
    let mut path = PathBuf::new();
    #[cfg(target_os = "windows")]
    path.push(std::env::var("HOMEPATH").unwrap());
    #[cfg(not(target_os = "windows"))]
    path.push(std::env::var("HOME").unwrap());
    path.push(".config");
    path.push("helix");
    path.push("languages.toml");
    path
}

fn get_default_npm_folder() -> PathBuf {
    let mut path = PathBuf::new();
    #[cfg(target_os = "windows")]
    path.push(std::env::var("HOMEPATH").unwrap());
    #[cfg(not(target_os = "windows"))]
    path.push(std::env::var("HOME").unwrap());
    path.push(".nvm");
    path.push("versions");
    path.push("node");
    path.push("v20.14.0");
    path.push("lib");
    path.push("node_modules");
    path
}

/// All the commands
#[derive(Subcommand)]
enum Comandos {
    /// Install LSP (Alias 'i')
    #[clap(alias = "i")]
    Install {
        #[clap()]
        name: String,
    },

    /// Remove a LSP (Alias 'r')
    #[clap(alias = "r")]
    Remove {
        #[clap()]
        name: String,
    },
}

fn main() {
    let args = Argumentos::parse();

    match args.cmd {
        Comandos::Install { name } => {
            if let Some(path_hx) = args.config.as_ref() {
                if name == "rust" {
                    languages::rust::add_rust(path_hx)
                } else if name == "typescript" {
                    if let Some(path_npm) = args.npm_folder.as_ref() {
                        languages::typescript::add_typescript(path_hx, path_npm)
                    }
                } else if name == "go" {
                    languages::go::add_go(path_hx)
                }
            }
        }
        Comandos::Remove { name } => {
            println!("Desinstalando: {name}")
        }
    }
}
