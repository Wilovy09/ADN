use crate::utilities::read;
use std::path::PathBuf;
use std::process::Command;

pub fn add_typescript(helix_lang_conf: &PathBuf, npm_folder: &PathBuf) {
    // crear una carpeta .npm-packages/
    let output = Command::new("npm")
        .arg("install")
        .arg("-g")
        .arg("typescript")
        .arg("typescript-language-server")
        .env("NODE_PATH", npm_folder)
        .spawn()
        .expect("\n-Falló al ejecutar `npm install -g typescript typescript-language-server`\n-Verifique que NPM este instalado");
    let output_err = output.wait_with_output().unwrap();
    if output_err.status.success() {
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
        let stderr = String::from_utf8_lossy(&output_err.stdout);
        eprintln!("Error al ejecutar el comando:\n{stderr}");
    }
}
