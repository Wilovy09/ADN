pub mod utilities;
pub mod languages;
use std::env;

fn print_usage() {
    println!("Ussage [PATH_TO_HELIX_LANGUAGES_FILE] install [LANGUAGE]");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 || args.iter().skip(1).any(|arg| arg.is_empty()) {
        print_usage();
        return;
    }

    let helix_lang_conf = &args[1];
    let action = &args[2];
    let lsp = &args[3];

    if !helix_lang_conf.ends_with("languages.toml") {
        print_usage();
        std::process::exit(1)
    }

    if action == "install" && lsp == "rust" {
        languages::rust::rust::add_rust(&helix_lang_conf);
    }
}
