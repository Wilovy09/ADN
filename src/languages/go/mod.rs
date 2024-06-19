use crate::utilities::read;
use std::path::PathBuf;
use std::process::Command;

pub fn add_go(helix_lang_conf: &PathBuf) {
    let output = Command::new("go")
        .arg("install")
        .arg("golang.org/x/tools/gopls@latest")
        .output()
        .expect("\nFallo al ejecutar 'go install golang.org/x/tools/gopls@latest'");

    if output.status.success() {
        println!("Instalación de 'go install golang.org/x/tools/gopls@latest' completa");

        let config_to_add = r#"
[languages.go]
language-server = { command = "gopls" }"#;
        if let Err(err) = read(helix_lang_conf, config_to_add) {
            eprintln!("Error al leer el archivo: {err}");
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error al ejecutar el comando:\n{stderr}")
    }
}
