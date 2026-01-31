use serde::{Deserialize, Serialize};
use std::{
    fmt, fs,
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
    pub id: i32,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (folder: {})", self.name, self.is_folder)
    }
}

impl File {
    pub fn get_file(&self, id: i32) -> Option<&File> {
        if self.id == id {
            return Some(self);
        }
        for child in &self.children {
            if let Some(file) = child.get_file(id) {
                return Some(file);
            }
        }
        None
    }
}

impl Project {
    pub fn new(name: &str, path: &PathBuf) -> Self {
        Project {
            name: name.to_string(),
            path: path.to_path_buf(),
            files: File {
                name: "Root".to_string(),
                children: Vec::new(),
                is_folder: true,
                path: path.to_path_buf(),
                id: 0,
            },
        }
    }

    pub fn from(path: &PathBuf) -> Result<Self, ProjectError> {
        let name = path
            .file_name()
            .ok_or(ProjectError::InvalidPath)?
            .to_string_lossy()
            .into_owned();
        Ok(Project::new(&name, path))
    }

    pub fn try_from_path(path: &PathBuf) -> Result<Self, ProjectError> {
        let name = path
            .file_name()
            .ok_or(ProjectError::InvalidPath)?
            .to_string_lossy()
            .into_owned();

        Ok(Project::new(&name, path))
    }

    pub fn get_file(&self, id: i32) -> Option<&File> {
        if self.files.id == id {
            return Some(&self.files);
        }
        self.files.get_file(id)
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
        let mut next_id = 1;
        self.files.children = self.load_directory(&self.path, &mut next_id)?;
        println!("Total files loaded: {}", self.files.children.len());

        Ok(())
    }

    fn load_directory(&self, path: &Path, next_id: &mut i32) -> Result<Vec<File>, ProjectError> {
        let mut files = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let fpath = entry.path();
            let is_folder = fpath.is_dir();

            let current_id = *next_id;
            *next_id += 1;

            let mut file = File {
                name: fpath
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned(),
                children: Vec::new(),
                is_folder,
                path: fpath.clone(),
                id: current_id,
            };

            if is_folder {
                file.children = self.load_directory(&fpath, next_id)?;
            }

            /*
            println!(
                "Found: {} (folder: {}, id: {})",
                file.name, file.is_folder, file.id
            );
            */
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
                id: 0,
            },
        }
    }
}
