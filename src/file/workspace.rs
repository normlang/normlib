use std::fs;

pub fn create(name: &str) -> std::io::Result<()> {
    fs::create_dir(name)?;
    Ok(())
}
