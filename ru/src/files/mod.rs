use image::DynamicImage;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::{ffi::OsStr, io::Write};
#[derive(Debug, Clone)]
pub enum FileType {
    Image,
}
pub struct FileSystem;
impl FileSystem {
    pub fn image_open(path: &Path) -> Result<DynamicImage, image::ImageError> {
        match image::open(path) {
            Ok(image) => Ok(image),
            Err(e) => return Err(e),
        }
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
}
