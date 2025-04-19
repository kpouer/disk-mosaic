use std::path::Path;
use unicode_normalization::UnicodeNormalization;

pub(crate) trait PathBufToString {
    fn name(&self) -> Option<String>;
    fn absolute_path(&self) -> Option<String>;
}

impl PathBufToString for Path {
    fn name(&self) -> Option<String> {
        self.file_name()
            .and_then(|f| f.to_str())
            .map(|f| f.nfc().collect::<String>())
    }

    fn absolute_path(&self) -> Option<String> {
        Some(self.as_os_str().to_str()?.nfc().collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_name() {
        let path = PathBuf::from("test.txt");
        assert_eq!(path.name(), Some("test.txt".nfc().collect()));
    }

    #[test]
    fn test_absolute_path() {
        let path = PathBuf::from("/home/user/test.txt");
        assert_eq!(
            path.absolute_path(),
            Some("/home/user/test.txt".nfc().collect())
        );
    }

    #[test]
    fn test_name_with_unicode() {
        let path = PathBuf::from("tést.txt");
        assert_eq!(path.name(), Some("tést.txt".nfc().collect()));
    }

    #[test]
    fn test_absolute_path_with_unicode() {
        let path = PathBuf::from("/home/user/tést.txt");
        assert_eq!(
            path.absolute_path(),
            Some("/home/user/tést.txt".nfc().collect())
        );
    }
}
