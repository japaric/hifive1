use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    fs::write(out_dir.join("link.x"), &include_bytes!("link.x")[..])?;

    println!("cargo:rustc-link-search={}", out_dir.display());

    Ok(())
}
