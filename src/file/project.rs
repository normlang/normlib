use std::fs;

pub const BUILD_DIR: &'static str = "build";

pub enum Target {
    Application,
    Library,
}

pub fn create(name: &str, target: Target) -> std::io::Result<()> {
    fs::create_dir(name)?;
    let file_name = String::from(name) + "/" + name + ".norm";

    match target {
        Target::Application => {
            fs::write(file_name, "@app")?;
        }
        Target::Library => {
            fs::write(file_name, "@lib")?;
        }
    }

    Ok(())
}

pub fn build() -> std::io::Result<()> {
    if !fs::exists(BUILD_DIR)? {
        fs::create_dir(BUILD_DIR)?;
    }

    Ok(())
}

pub fn run() -> std::io::Result<()> {
    println!("Application runned...");
    Ok(())
}
