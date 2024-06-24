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

    /// This is for 'node_modules/'
    #[clap(long, short)]
    #[arg(default_value=get_default_npm_folder().into_os_string())]
    npm_folder: PathBuf,

    #[clap(subcommand)]
    cmd: Comandos,
}

#[cfg(target_os = "windows")]
fn get_default_config_path() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(std::env::var("APPDATA").uwrap());
    path.push("helix");
    path.push("languages.toml");
    path
}
#[cfg(not(target_os = "windows"))]
fn get_default_config_path() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(std::env::var("HOME").unwrap());
    path.push(".config");
    path.push("helix");
    path.push("languages.toml");
    path
}

#[cfg(target_os = "windows")]
fn get_default_npm_folder() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(std::env::var("PROGRAMFILES").unwrap());
    path.push("nodejs");
    path.push("node_modules");
    path
}
#[cfg(not(target_os = "windows"))]
fn get_default_npm_folder() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(std::env::var("HOME").unwrap());
    path.push(".nvm");
    path.push("versions");
    path.push("node");
    path.push("v20.15.0");
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

    /// WIP Remove a LSP (Alias 'r')
    #[clap(alias = "r")]
    Remove {
        #[clap(subcommand)]
        language: Language,
    },
}

#[derive(Subcommand)]
enum Language {
    Rust,
    Typescript,
    Go,
    Python,
    Html,
    Css,
    Javascript,
    Jsx,
    Tsx,
    Json,
}

fn main() {
    let args = Argumentos::parse();
    match args.cmd {
        Comandos::Install { language } => match language {
            Language::Rust => languages::rust::add_rust(&args.config),
            Language::Typescript => {
                languages::typescript::add_typescript(&args.config, &args.npm_folder)
            }
            Language::Go => languages::go::add_go(&args.config),
            Language::Python => languages::python::add_python(&args.config),
            Language::Html => languages::html::add_html(&args.config, &args.npm_folder),
            Language::Css => languages::css::add_css(&args.config, &args.npm_folder),
            Language::Javascript => {
                languages::javascript::add_javascript(&args.config, &args.npm_folder)
            }
            Language::Jsx => languages::jsx::add_jsx(&args.config, &args.npm_folder),
            Language::Tsx => languages::tsx::add_tsx(&args.config, &args.npm_folder),
            Language::Json => languages::json::add_json(&args.config, &args.npm_folder),
        },
        Comandos::Remove { .. } => {
            println!("Desinstalando... (WIP)")
        }
    }
}
