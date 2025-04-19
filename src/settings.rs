use egui::Context;
use log::info;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Settings {
    #[serde(skip)]
    /// Mark the Settings as dirty (need to be saved)
    dirty: bool,
    theme: ThemePreference,
    /// List of paths to ignore (might be cloud drives, etc.
    ignored_path: Vec<PathBuf>,
}

impl Default for Settings {
    fn default() -> Self {
        Self::settings_file()
            .and_then(|settings_file| File::open(settings_file).ok())
            .and_then(|settings_file| serde_json::from_reader::<File, Settings>(settings_file).ok())
            .unwrap_or(Self {
                dirty: false,
                theme: ThemePreference::System,
                ignored_path: Vec::new(),
            })
    }
}

impl Settings {
    pub(crate) fn theme(&self) -> ThemePreference {
        self.theme
    }

    pub(crate) fn set_theme(&mut self, theme: ThemePreference) {
        self.theme = theme;
        self.dirty = true;
    }

    pub(crate) fn init(&self, ctx: &Context) {
        ctx.set_theme(self.theme);
    }

    pub(crate) fn add_ignored_path(&mut self, path: PathBuf) {
        self.ignored_path.push(path);
    }

    pub(crate) fn is_path_ignored(&self, path: &PathBuf) -> bool {
        self.ignored_path.contains(path)
    }

    pub(crate) fn ignored_paths_mut(&mut self) -> &mut Vec<PathBuf> {
        &mut self.ignored_path
    }

    pub(crate) fn save(&self) -> Result<(), std::io::Error> {
        info!("save");
        if self.dirty {
            if let Some(settings_folder) = Self::settings_folder() {
                std::fs::create_dir_all(settings_folder)?;
                if let Some(settings_file) = Self::settings_file() {
                    serde_json::to_writer(File::create(settings_file)?, self)?;
                }
            }
        }
        Ok(())
    }

    fn settings_folder() -> Option<PathBuf> {
        home::home_dir().map(|mut home| {
            home.push(".disk-analyzer");
            home
        })
    }

    fn settings_file() -> Option<PathBuf> {
        Self::settings_folder().map(|mut settings_folder| {
            settings_folder.push("settings.json");
            settings_folder
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ThemePreference {
    #[default]
    System,
    Dark,
    Light,
}

impl From<ThemePreference> for egui::ThemePreference {
    fn from(theme: ThemePreference) -> Self {
        match theme {
            ThemePreference::System => egui::ThemePreference::System,
            ThemePreference::Dark => egui::ThemePreference::Dark,
            ThemePreference::Light => egui::ThemePreference::Light,
        }
    }
}
