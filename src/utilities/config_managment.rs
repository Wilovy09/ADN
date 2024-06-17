use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub fn read(read_file: &PathBuf, content_lsp: &str) -> io::Result<String> {
    let content = File::open(read_file)
        .and_then(|mut file| {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            Ok(content)
        })
        .unwrap_or_else(|_| String::new());
    let content_lsp_to_func = content_lsp;
    match add_config_lsp(read_file, content_lsp_to_func) {
        Ok(_) => println!("Config added correctly."),
        Err(e) => eprintln!("Error: {e}"),
    }
    Ok(content)
}

pub fn add_config_lsp(filename: &PathBuf, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;
    writeln!(file, "{content}")?;
    Ok(())
}
