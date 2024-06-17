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
    config: PathBuf,

    #[clap(subcommand)]
    cmd: Comandos,
}

fn get_default_config_path() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(env!("HOME"));
    path.push(".config");
    path.push("helix");
    path.push("languages.toml");
    path
}

fn get_default_npm_folder() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(env!("HOME"));
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
        #[clap(subcommand)]
        language: Language,
    },

    /// Remove a LSP (Alias 'r')
    #[clap(alias = "r")]
    Remove {
        #[clap(subcommand)]
        language: Language,
    },
}

#[derive(Subcommand)]
enum Language {
    Rust,
    Typescript {
        /// This is for 'node_modules/'
        #[clap(long, short)]
        #[arg(default_value=get_default_npm_folder().into_os_string())]
        npm_folder: PathBuf,
    },
}

fn main() {
    let args = Argumentos::parse();

    match args.cmd {
        Comandos::Install { language } => match language {
            Language::Rust => languages::rust::add_rust(&args.config),
            Language::Typescript { npm_folder } => {
                languages::typescript::add_typescript(&args.config, &npm_folder)
            }
        },
        Comandos::Remove { .. } => {
            println!("Desinstalando")
        }
    }
}
