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
        let mut absolute_path = String::with_capacity(100);
        for ancestor in self.ancestors() {
            absolute_path.push('/');
            absolute_path.push_str(&ancestor.name().unwrap_or_default());
        }
        Some(absolute_path)
    }
}
