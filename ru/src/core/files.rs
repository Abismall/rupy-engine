use image::DynamicImage;
use std::fs::{self, File};
use std::io::{self};
use std::path::{Path, PathBuf};
use std::{ffi::OsStr, io::Write};

use crate::{log_error, log_info};

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
    pub fn get_assets_dir() -> Result<PathBuf, AppError> {
        let base_dir = std::env::current_dir()?.join("src").join("assets");
        Ok(base_dir)
    }

    pub fn get_res_dir() -> Result<PathBuf, AppError> {
        let path = Self::get_assets_dir()?.join("res");
        Ok(path)
    }

    pub fn get_scenes_dir() -> Result<PathBuf, AppError> {
        log_info!("get_scenes_dir");
        let path = Self::get_assets_dir()?.join("scenes");
        log_info!("scenes_dir {:?}", path);
        Ok(path)
    }

    pub fn get_textures_dir() -> Result<PathBuf, AppError> {
        let path = Self::get_assets_dir()?.join("textures");
        Ok(path)
    }

    pub fn get_shaders_dir() -> Result<PathBuf, AppError> {
        let path = Self::get_assets_dir()?.join("shaders");
        Ok(path)
    }

    pub fn get_texture_base_folder(file_name: &str) -> Result<PathBuf, AppError> {
        log_info!("get_texture_base_folder: {:?}", file_name);
        let path = Self::get_textures_dir()?.join(file_name);
        Ok(path)
    }

    pub fn get_shader_file_path(file_name: &str) -> Result<PathBuf, AppError> {
        log_info!("get_shader_file_path: {:?}", file_name);
        let path = Self::get_shaders_dir()?.join(file_name);
        Ok(path)
    }

    pub fn get_scene_file_path(file_name: &str) -> Result<PathBuf, AppError> {
        log_info!("get_scene_file_path: {:?}", file_name);
        let path = Self::get_scenes_dir()?.join(file_name);
        Ok(path)
    }

    pub fn load_string(file_name: &str) -> Result<String, AppError> {
        let path = Self::get_res_dir()?.join(file_name);
        let txt = Self::read_to_string(path)?;
        Ok(txt)
    }

    pub fn load_binary(file_name: &str) -> Result<Vec<u8>, AppError> {
        let path = Self::get_res_dir()?.join(file_name);
        let data = std::fs::read(path)?;
        Ok(data)
    }

    pub fn image_open(path: &Path) -> Result<DynamicImage, image::ImageError> {
        match image::open(path) {
            Ok(image) => Ok(image),
            Err(e) => {
                log_error!("Failed to open image: {:?}", e);
                return Err(e);
            }
        }
    }
    pub fn get_path_for_type(file_type: FileType, file_name: &str) -> Result<PathBuf, AppError> {
        let base_dir = match file_type {
            FileType::Image => Self::get_textures_dir()?,
        };
        Ok(base_dir.join(file_name))
    }

    pub fn load_or_create<F>(
        file_type: FileType,
        file_name: &str,
        create_fn: F,
    ) -> Result<String, AppError>
    where
        F: FnOnce(&Path) -> Result<String, AppError>,
    {
        let path = Self::get_path_for_type(file_type, file_name)?;
        if path.exists() {
            log_info!("File exists, reading: {:?}", path);
            return Self::read_to_string(&path).map_err(|e| e.into());
        }

        log_info!("File does not exist, creating: {:?}", path);
        let content = create_fn(&path)?;
        Self::write_to_file(&path, &content)?;
        Ok(content)
    }
    pub fn read_image_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, image::ImageError> {
        let image = image::open(path.as_ref())?;
        let rgba_image = image.to_rgba8();
        Ok(rgba_image.into_raw())
    }

    pub fn read_to_string<P: AsRef<Path> + std::fmt::Debug>(path: P) -> io::Result<String> {
        let content = fs::read_to_string(path.as_ref());
        if !content.is_ok() {
            log_error!("Failed to read file: {:?}", path);
        }
        content
    }

    pub fn write_to_file<P: AsRef<Path> + std::fmt::Debug, S: AsRef<str>>(
        path: P,
        content: S,
    ) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(content.as_ref().as_bytes())?;
        Ok(())
    }

    pub fn list_files_in_dir<P: AsRef<Path> + std::fmt::Debug>(
        path: P,
    ) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().is_file() {
                files.push(entry.path());
            }
        }
        Ok(files)
    }

    pub fn list_files_with_extension<P: AsRef<Path> + std::fmt::Debug, E: AsRef<OsStr>>(
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
}
