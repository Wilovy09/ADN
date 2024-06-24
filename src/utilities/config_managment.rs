use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub fn read(read_file: &PathBuf, content_lsp: &str) -> io::Result<String> {
    if !read_file.exists() {
        create_file(read_file)?;
    }

    let mut file = File::open(read_file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    match add_config_lsp(read_file, content_lsp) {
        Ok(_) => println!("Config added correctly."),
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(content)
}

pub fn add_config_lsp(filename: &Path, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;
    writeln!(file, "{}", content)?;
    Ok(())
}

fn create_file(path: &PathBuf) -> io::Result<()> {
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)?;
    }
    File::create(path)?;
    Ok(())
}
