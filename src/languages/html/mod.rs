use crate::utilities::read;
use std::path::PathBuf;
use std::process::Command;

pub fn add_html(helix_lang_conf: &PathBuf, npm_folder: &PathBuf) {
    let output = Command::new("npm")
        .arg("install")
        .arg("-g")
        .arg("vscode-langservers-extracted")
        .arg("@olrtg/emmet-language-server")
        .env("NODE_PATH", npm_folder)
        .spawn()
        .expect("\nFallo la instalación al ejecutar 'npm install -g vscode-langservers-extracted @olrtg/emmet-language-server'");
    let output_err = output.wait_with_output().unwrap();
    if output_err.status.success() {
        println!("Instalación de `npm install -g vscode-langservers-extracted @olrtg/emmet-language-server
` completa");

        let config_to_add = r#"
[language-server.emmet-lsp]
command = "emmet-language-server"
args = ["--stdio"]

[[language]]
name = "html"
roots = [".git"]
language-servers = ["emmet-lsp", "vscode-html-language-server"]"#;
        if let Err(err) = read(helix_lang_conf, config_to_add) {
            eprintln!("Error al leer el archivo: {err}");
        }
    } else {
        let stderr = String::from_utf8_lossy(&output_err.stdout);
        eprintln!("Error al ejecutar el comando:\n{stderr}");
    }
}
