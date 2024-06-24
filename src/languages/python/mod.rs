use crate::utilities::read;
use std::path::PathBuf;
use std::process::Command;

pub fn add_python(helix_lang_conf: &PathBuf) {
    // pip install 'python-lsp-server[all]'
    let output = Command::new("pip")
        .arg("install")
        .arg("python-lsp-server[all]")
        .arg("--break-system")
        .arg("packages")
        .output()
        .expect(
            "\nFalló al instalar pip install 'python-lsp-server[all]'
",
        );
    if output.status.success() {
        println!(
            "Instalacion de pip install 'python-lsp-server[all]'
 exitosa."
        );
        let config_to_add = r#"
[language.python]
language-servers = ['pylsp']    

[language-server.pylsp.config.pylsp]
plugins.pyls_mypy.enabled = true
plugins.pyls_mypy.live_mode = true"#;
        if let Err(err) = read(helix_lang_conf, config_to_add) {
            eprintln!("Error al leer el archivo: {err}");
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "Error al ejecutar el comando: pip install 'python-lsp-server[all]'\nError:\n{stderr}"
        )
    }
}
