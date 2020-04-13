use std::path::PathBuf;
use serde_derive::{Deserialize, Serialize};
use directories::ProjectDirs;

use crate::record::Record;


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub records: Vec<Record>,
}


#[derive(Debug, Clone)]
pub enum LoadError {
    FileError,
    FormatError,
}


#[derive(Debug, Clone)]
pub enum SaveError {
    DirectoryError,
    FileError,
    WriteError,
    FormatError,
}


// modified from 'iced/example/todos'
impl AppData {
    fn path() -> PathBuf {
        let mut path = match ProjectDirs::from("rs", "evolix1", "Crash Recorder") {
            Some(project_dirs) => project_dirs.data_dir().into(),
            None => std::env::current_dir().unwrap_or(PathBuf::new())
        };

        path.push("records.json");

        path
    }

    pub async fn load() -> Result<AppData, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::FileError)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::FileError)?;

        serde_json::from_str(&contents)
            .map_err(|_| LoadError::FormatError)
    }

    pub async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;

        let json = serde_json::to_string_pretty(&self)
            .map_err(|_| SaveError::FormatError)?;

        let path = Self::path();

        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::DirectoryError)?;
        }

        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::FileError)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::WriteError)?;
        }

        // This is a simple way to save at most once every couple seconds
        async_std::task::sleep(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}
