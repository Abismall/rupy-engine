use image::DynamicImage;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::{ffi::OsStr, io::Write};

use crate::log_error;

use super::error::AppError;
#[derive(Debug, Clone)]
pub struct PathBuilder {
    pub path: PathBuf,
}

impl PathBuilder {
    pub fn new(base: PathBuf) -> Self {
        Self { path: base }
    }

    pub fn push<T: AsRef<Path>>(mut self, component: T) -> Self {
        self.path.push(component);
        self
    }

    pub fn build(self) -> String {
        self.path.display().to_string()
    }
}
#[derive(Debug, Clone, Copy)]
pub enum FileType {
    Image,
}
pub struct FileSystem;
impl FileSystem {
    pub fn get_static_dir() -> Result<PathBuf, AppError> {
        std::env::var("RUPY_ENGINE_STATIC_DIR")
            .map(PathBuf::from)
            .map_err(|e| AppError::ConfigError(e.to_string()))
    }
    pub fn get_scenes_dir() -> Result<PathBuf, AppError> {
        Ok(FileSystem::join_paths([
            FileSystem::get_static_dir()?,
            "scenes".into(),
        ]))
    }
    pub fn get_textures_dir() -> Result<PathBuf, AppError> {
        std::env::var("RUPY_ENGINE_TEXTURES_DIR")
            .map(PathBuf::from)
            .map_err(|e| AppError::ConfigError(e.to_string()))
    }

    pub fn get_shaders_dir() -> Result<PathBuf, AppError> {
        std::env::var("RUPY_ENGINE_SHADERS_DIR")
            .map(PathBuf::from)
            .map_err(|e| AppError::ConfigError(e.to_string()))
    }
    pub fn get_texture_base_folder(file_name: &str) -> Result<PathBuf, AppError> {
        Ok(FileSystem::get_textures_dir()?.join(file_name))
    }

    pub fn get_shader_file_path(file_name: &str) -> Result<PathBuf, AppError> {
        Ok(FileSystem::get_shaders_dir()?.join(file_name))
    }
    pub fn get_scene_file_path(file_name: &str) -> Result<PathBuf, AppError> {
        Ok(FileSystem::get_scenes_dir()?.join(file_name))
    }
    pub fn resolve_static_path(path: &str) -> Result<PathBuf, AppError> {
        let resolved_path = fs::canonicalize(PathBuf::from(path))?;
        Ok(resolved_path)
    }

    pub fn image_open(path: &Path) -> Result<DynamicImage, image::ImageError> {
        match image::open(path) {
            Ok(image) => Ok(image),
            Err(e) => return Err(e),
        }
    }
    pub fn read_image_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, image::ImageError> {
        let image = image::open(path)?;
        let rgba_image = image.to_rgba8();
        Ok(rgba_image.into_raw())
    }
    pub fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
        fs::read_to_string(path)
    }

    pub fn read_to_bytes<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
        fs::read(path)
    }

    pub fn write_to_file<P: AsRef<Path>, S: AsRef<str>>(path: P, content: S) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(content.as_ref().as_bytes())?;
        Ok(())
    }

    pub fn append_to_file<P: AsRef<Path>, S: AsRef<str>>(path: P, content: S) -> io::Result<()> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        file.write_all(content.as_ref().as_bytes())?;
        Ok(())
    }

    pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
        Path::new(path.as_ref()).exists()
    }

    pub fn create_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    pub fn delete_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
        fs::remove_file(path)
    }

    pub fn delete_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
        fs::remove_dir_all(path)
    }

    pub fn list_files_in_dir<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().is_file() {
                files.push(entry.path());
            }
        }
        Ok(files)
    }
    pub fn load_image_file(file_path: &PathBuf) -> Result<DynamicImage, AppError> {
        match image::open(file_path) {
            Ok(texture) => Ok(texture),
            Err(e) => {
                log_error!("Failed to load texture file: {}", e.to_string());
                Err(AppError::ImageError(e))
            }
        }
    }

    pub fn list_files_with_extension<P: AsRef<Path>, E: AsRef<OsStr>>(
        path: P,
        extension: E,
    ) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension() == Some(extension.as_ref()) {
                files.push(path);
            }
        }
        Ok(files)
    }

    pub fn copy_file<P: AsRef<Path>>(source: P, destination: P) -> io::Result<u64> {
        fs::copy(source, destination)
    }

    pub fn move_file<P: AsRef<Path>>(source: P, destination: P) -> io::Result<()> {
        fs::rename(source, destination)
    }

    pub fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        reader.lines().collect()
    }

    pub fn write_lines<P: AsRef<Path>>(path: P, lines: &[String]) -> io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        for line in lines {
            writeln!(writer, "{}", line)?;
        }
        Ok(())
    }
    pub fn join_paths<I, P>(paths: I) -> PathBuf
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let mut joined_path = PathBuf::new();
        for path in paths {
            joined_path.push(path.as_ref());
        }
        joined_path
    }
}
