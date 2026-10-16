use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Project {
    _name: String,
    _path: PathBuf,
    _target: Target,
}

pub enum Target {
    Application,
    Library,
}

impl Project {
    pub const BUILD_DIR: &'static str = "build";

    pub fn create(
        path: impl AsRef<Path>,
        name: &str,
        target: Target,
    ) -> Result<Project, std::io::Error> {
        let file_name = path.as_ref().join(name.to_string() + ".norm");
        println!("filename {}", file_name.to_str().unwrap());

        match target {
            Target::Application => {
                fs::write(&file_name, "@app")?;
            }
            Target::Library => {
                fs::write(&file_name, "@lib")?;
            }
        }

        Ok(Project {
            _name: name.to_string(),
            _target: target,
            _path: file_name,
        })
    }
}

pub fn build() -> std::io::Result<()> {
    if !fs::exists(Project::BUILD_DIR)? {
        fs::create_dir(Project::BUILD_DIR)?;
    }

    Ok(())
}

pub fn run() -> std::io::Result<()> {
    println!("Application runned...");
    Ok(())
}
