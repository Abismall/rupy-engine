use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

pub struct ShaderSourcePath<'a> {
    inner: Cow<'a, Path>,
}

impl<'a> ShaderSourcePath<'a> {
    /// Create a new `ShaderSourcePath` from a string slice
    pub fn from_str(path: &'a str) -> Self {
        ShaderSourcePath {
            inner: Cow::Borrowed(Path::new(path)),
        }
    }

    /// Create a new `ShaderSourcePath` from a `String`
    pub fn from_string(path: String) -> Self {
        ShaderSourcePath {
            inner: Cow::Owned(PathBuf::from(path)),
        }
    }

    /// Create a new `ShaderSourcePath` from a `PathBuf`
    pub fn from_pathbuf(path: PathBuf) -> Self {
        ShaderSourcePath {
            inner: Cow::Owned(path),
        }
    }

    /// Get the path as a `PathBuf`
    pub fn to_pathbuf(&self) -> PathBuf {
        self.inner.clone().into_owned()
    }

    /// Get a reference to the `Path`
    pub fn as_path(&self) -> &Path {
        &self.inner
    }
}

impl<'a> From<&'a str> for ShaderSourcePath<'a> {
    fn from(path: &'a str) -> Self {
        ShaderSourcePath::from_str(path)
    }
}

impl From<String> for ShaderSourcePath<'_> {
    fn from(path: String) -> Self {
        ShaderSourcePath::from_string(path)
    }
}

impl From<PathBuf> for ShaderSourcePath<'_> {
    fn from(path: PathBuf) -> Self {
        ShaderSourcePath::from_pathbuf(path)
    }
}
