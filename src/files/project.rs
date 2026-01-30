use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum ProjectError {
    InvalidPath,
    IoError(std::io::Error),
}

impl From<std::ffi::OsString> for ProjectError {
    fn from(_: std::ffi::OsString) -> Self {
        ProjectError::InvalidPath
    }
}

impl From<std::io::Error> for ProjectError {
    fn from(err: std::io::Error) -> Self {
        ProjectError::IoError(err)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub files: File,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub name: String,
    pub children: Vec<File>,
    pub is_folder: bool,
    pub path: PathBuf,
}

impl Project {
    pub fn new(name: &str, path: &Path) -> Self {
        Project {
            name: name.to_string(),
            path: path.to_path_buf(),
            files: File {
                name: "Root".to_string(),
                children: Vec::new(),
                is_folder: true,
                path: path.to_path_buf(),
            },
        }
    }

    pub fn from(path: &Path) -> Result<Self, ProjectError> {
        let name = path
            .file_name()
            .ok_or(ProjectError::InvalidPath)?
            .to_string_lossy()
            .into_owned();
        Ok(Project::new(&name, path))
    }

    pub fn get_file(&self, path: String) -> Option<&File> {
        for file in &self.files.children {
            if file.is_folder {
                if let Some(child) = file.get_file(path.clone()) {
                    return Some(child);
                }
            } else if file.path == path {
                Some()
            }
        }

        None
    }

    pub fn load(mut self) -> Self {
        if let Err(e) = self.load_files() {
            eprintln!("Error loading files: {:?}", e);
        }
        self
    }

    pub fn load_files(&mut self) -> Result<(), ProjectError> {
        if !self.path.exists() {
            eprintln!("Path does not exist: {:?}", self.path);
            return Ok(());
        }

        if !self.path.is_dir() {
            eprintln!("Path is not a directory: {:?}", self.path);
            return Ok(());
        }

        println!("Loading files from: {:?}", self.path);
        self.files.children = self.load_directory(&self.path)?;
        println!("Total files loaded: {}", self.files.children.len());
        Ok(())
    }

    fn load_directory(&self, path: &Path) -> Result<Vec<File>, ProjectError> {
        let mut files = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let fpath = entry.path();
            let is_folder = fpath.is_dir();

            let mut file = File {
                name: fpath
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned(),
                children: Vec::new(),
                is_folder,
                path: fpath.clone(),
            };

            if is_folder {
                file.children = self.load_directory(&fpath)?;
            }

            println!("Found: {} (folder: {})", file.name, file.is_folder);
            files.push(file);
        }

        Ok(files)
    }
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            path: PathBuf::from("."),
            files: File {
                name: "Root".to_string(),
                children: Vec::new(),
                is_folder: true,
                path: PathBuf::from("."),
            },
        }
    }
}
