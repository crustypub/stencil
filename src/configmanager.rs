use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct FileRegistry {
    files: HashMap<String, PathBuf>, // имя файла -> полный путь
}

pub struct ConfigManager {
    registry: FileRegistry,
    registry_path: PathBuf,
    config_dir: PathBuf,
}

impl ConfigManager {
    pub fn new() -> io::Result<Self> {
        let proj_dirs = ProjectDirs::from("com", "mycompany", "myapp")
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No home directory"))?;

        let config_dir = proj_dirs.config_dir().to_path_buf();
        fs::create_dir_all(&config_dir)?;

        let registry_path = config_dir.join("file_registry.json");

        let registry = if registry_path.exists() {
            let content = fs::read_to_string(&registry_path)?;
            serde_json::from_str(&content).unwrap_or_else(|_| FileRegistry {
                files: HashMap::new(),
            })
        } else {
            FileRegistry {
                files: HashMap::new(),
            }
        };

        Ok(Self {
            registry,
            registry_path,
            config_dir,
        })
    }

    pub fn save_config_file(&mut self, original_path: &Path) -> io::Result<PathBuf> {
        let file_name = original_path
            .file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file path"))?
            .to_string_lossy()
            .to_string();

        let target_path = self.config_dir.join(&file_name);

        // Копируем файл
        fs::copy(original_path, &target_path)?;

        // Сохраняем в реестр
        self.registry
            .files
            .insert(file_name.clone(), target_path.clone());
        self.save_registry()?;

        Ok(target_path)
    }

    pub fn get_config_file(&self, file_name: &str) -> Option<&Path> {
        self.registry.files.get(file_name).map(|p| p.as_path())
    }

    pub fn read_config_file(&self, file_name: &str) -> io::Result<String> {
        let path = self.get_config_file(file_name).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("File '{}' not found in registry", file_name),
            )
        })?;

        fs::read_to_string(path)
    }

    fn save_registry(&self) -> io::Result<()> {
        let content = serde_json::to_string_pretty(&self.registry)?;
        fs::write(&self.registry_path, content)
    }
}
