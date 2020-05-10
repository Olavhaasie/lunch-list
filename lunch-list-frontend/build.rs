use sass_rs::{Options, OutputStyle};
use std::{fs::write, io::{Error, ErrorKind}, path::PathBuf};

const CSS_FILE: &str = "static/styles.css";
const SASS_FILE: &str = "src/styles.scss";

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed={}", SASS_FILE);

    let options = Options { output_style: OutputStyle::Compressed, .. Default::default() };
    let path = PathBuf::from(SASS_FILE);
    let css = sass_rs::compile_file(&path, options).map_err(|s| Error::new(ErrorKind::Other, s))?;

    let output_path = PathBuf::from(CSS_FILE);
    write(&output_path, css)?;

    Ok(())
}
