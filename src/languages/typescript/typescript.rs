use std::process::Command;
use crate::utilities::read;

pub fn add_typescript(helix_lang_conf: &str) {
    let output = Command::new("npm")
        .arg("install")
        .arg("-g")
        .arg("typescript")
        .arg("typescript-language-server")
        .output()
        .expect("\n-Falló al ejecutar `npm install -g typescript typescript-language-server`\n-Verifique que NPM este instalado");

    if output.status.success() {
        println!("Instalación de `npm install -g typescript typescript-language-server` completa");

        let config_to_add = r#"
[languages.typescript]
language-server = [
  { command = "typescript-language-server", args = "--stdio" }
]
"#;
        if let Err(err) = read(helix_lang_conf, config_to_add) {
            eprintln!("Error al leer el archivo: {err}");
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error al ejecutar el comando:\n{stderr}");
    }
}

