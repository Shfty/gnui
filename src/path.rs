use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct Path(Option<PathBuf>);

impl From<&OsStr> for Path {
    fn from(path: &OsStr) -> Self {
        match path.to_str().unwrap() {
            "-" => Path(None),
            input => Path(Some(input.into())),
        }
    }
}

impl From<Path> for Option<PathBuf> {
    fn from(path: Path) -> Self {
        path.0
    }
}
