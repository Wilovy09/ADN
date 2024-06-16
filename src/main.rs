use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::process::Command;

fn read(read_file: &str, content_lsp: &str) -> io::Result<String> {
    let mut file = File::open(read_file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let content_lsp_to_func = content_lsp;
    match add_config_lsp(read_file, content_lsp_to_func) {
        Ok(_) => println!("Contenido añadido correctamente."),
        Err(e) => eprintln!("Error al añadir config al archivo, {}", e),
    }
    Ok(content)
}

fn add_config_lsp(filename: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    writeln!(file, "{}", content)?;

    Ok(())
}

fn main() {
    // adn [config helix file path] install rust
    // ADN [config helix file path] install rust
    let args: Vec<String> = env::args().collect();

    let helix_lang_conf = &args[1];
    let action = &args[2];
    let lsp = &args[3];

    if action == "install" && lsp == "rust" {
        let output = Command::new("rustup")
            .arg("component")
            .arg("add")
            .arg("rust-analyzer")
            .output()
            .expect("Falló al ejecutar `rustup component add rust-analyzer`");

        if output.status.success() {
            println!("Instalación de `rustup component add rust-analyzer` completa");

            let config_to_add = r#"
# rustup component add rust-analyzer ----------ADN---------------------------------
[[language]]
name = "rust"

[language-server.rust-analyzer.config.check]
command = "clippy"
scope = "source.rust"
injection-regex = "rust"
file-types = ["rs"]
roots = ["Cargo.toml", "Cargo.lock"]
auto-format = true
comment-token = "//"
language-server = { command = "rust-analyzer" }
indent = { tab-width = 4, unit = "    " }
"#;
            if let Err(err) = read(helix_lang_conf, config_to_add) {
                eprintln!("Error al leer el archivo: {}", err);
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error al ejecutar el comando:\n{}", stderr);
        }
    }
}
