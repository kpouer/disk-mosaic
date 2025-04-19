use std::path::Path;
use thiserror::Error;
use unicode_normalization::UnicodeNormalization;

pub(crate) const FONT_SIZE: f32 = 18.0;

#[derive(Error, Debug)]
pub(crate) enum MyError {
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
    #[error("Receiver Dropped")]
    ReceiverDropped,
}

pub fn get_file_size(path: &Path) -> u64 {
    path.metadata().map(|metadata| metadata.len()).unwrap_or(0)
}

pub(crate) trait PathBufToString {
    fn name(&self) -> String;
    fn absolute_path(&self) -> String;
}

impl PathBufToString for Path {
    fn name(&self) -> String {
        self.file_name()
            .and_then(|f| f.to_str())
            .map(|f| f.nfc().collect::<String>())
            .unwrap_or_default()
    }

    fn absolute_path(&self) -> String {
        self.as_os_str()
            .to_str()
            .map(|f| f.nfc().collect::<String>())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_name() {
        let path = PathBuf::from("test.txt");
        assert_eq!(path.name(), "test.txt");
    }

    #[test]
    fn test_absolute_path() {
        let path = PathBuf::from("/home/user/test.txt");
        assert_eq!(path.absolute_path(), "/home/user/test.txt");
    }

    #[test]
    fn test_name_with_unicode() {
        let path = PathBuf::from("tést.txt");
        assert_eq!(path.name(), "tést.txt");
    }

    #[test]
    fn test_absolute_path_with_unicode() {
        let path = PathBuf::from("/home/user/tést.txt");
        assert_eq!(path.absolute_path(), "/home/user/tést.txt");
    }
}
