//! Configuration management for iced-py-gui
//! Handles loading, validating, and providing file filters from user config

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// File filter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileFilter {
    pub name: String,
    pub extensions: String,
}

/// Complete configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileFiltersConfig {
    pub filters: Vec<FileFilter>,
}

/// Cached configuration
static FILE_FILTERS_CACHE: Lazy<Mutex<Option<Vec<(String, String)>>>> = Lazy::new(|| Mutex::new(None));

/// Get the configuration directory path based on OS
fn get_config_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA")
            .unwrap_or_else(|_| String::from("."));
        PathBuf::from(appdata).join("ipg")
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| String::from("."));
        PathBuf::from(home).join("Library/Application Support/ipg")
    }

    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| String::from("."));
        let xdg_config = std::env::var("XDG_CONFIG_HOME").ok();
        
        if let Some(xdg) = xdg_config {
            PathBuf::from(xdg).join("ipg")
        } else {
            PathBuf::from(home).join(".config/ipg")
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        PathBuf::from(".").join(".ipg")
    }
}

/// Get the file filters config file path
fn get_config_file_path() -> PathBuf {
    get_config_dir().join("file_filters.json")
}

/// Default file filters
fn default_filters() -> FileFiltersConfig {
    FileFiltersConfig {
        filters: vec![
            // Document Formats
            FileFilter {
                name: "Text Files".to_string(),
                extensions: "*.txt".to_string(),
            },
            FileFilter {
                name: "PDF Files".to_string(),
                extensions: "*.pdf".to_string(),
            },
            FileFilter {
                name: "Word Documents".to_string(),
                extensions: "*.docx;*.doc".to_string(),
            },
            // Programming
            FileFilter {
                name: "Python Files".to_string(),
                extensions: "*.py".to_string(),
            },
            FileFilter {
                name: "Rust Files".to_string(),
                extensions: "*.rs".to_string(),
            },
            FileFilter {
                name: "JavaScript Files".to_string(),
                extensions: "*.js;*.jsx".to_string(),
            },
            FileFilter {
                name: "TypeScript Files".to_string(),
                extensions: "*.ts;*.tsx".to_string(),
            },
            FileFilter {
                name: "JSON Files".to_string(),
                extensions: "*.json".to_string(),
            },
            FileFilter {
                name: "YAML Files".to_string(),
                extensions: "*.yaml;*.yml".to_string(),
            },
            FileFilter {
                name: "HTML Files".to_string(),
                extensions: "*.html;*.htm".to_string(),
            },
            FileFilter {
                name: "CSS Files".to_string(),
                extensions: "*.css;*.scss;*.sass".to_string(),
            },
            // Images
            FileFilter {
                name: "Image Files".to_string(),
                extensions: "*.png;*.jpg;*.jpeg;*.gif;*.bmp;*.svg;*.webp".to_string(),
            },
            FileFilter {
                name: "PNG Images".to_string(),
                extensions: "*.png".to_string(),
            },
            FileFilter {
                name: "JPEG Images".to_string(),
                extensions: "*.jpg;*.jpeg".to_string(),
            },
            // Audio/Video
            FileFilter {
                name: "Audio Files".to_string(),
                extensions: "*.mp3;*.wav;*.aac;*.flac;*.ogg".to_string(),
            },
            FileFilter {
                name: "Video Files".to_string(),
                extensions: "*.mp4;*.avi;*.mkv;*.mov;*.flv".to_string(),
            },
            // Archives
            FileFilter {
                name: "Archive Files".to_string(),
                extensions: "*.zip;*.rar;*.7z;*.tar;*.gz".to_string(),
            },
            // All Files
            FileFilter {
                name: "All Files".to_string(),
                extensions: "*.*".to_string(),
            },
        ],
    }
}

/// Load filters from disk or create default config
pub fn load_file_filters() -> Result<Vec<(String, String)>, String> {
    // Check cache first
    if let Ok(cache) = FILE_FILTERS_CACHE.lock() {
        if let Some(filters) = cache.as_ref() {
            return Ok(filters.clone());
        }
    }

    let config_path = get_config_file_path();

    // Try to load from user config file
    if config_path.exists() {
        match fs::read_to_string(&config_path) {
            Ok(content) => {
                match serde_json::from_str::<FileFiltersConfig>(&content) {
                    Ok(config) => {
                        let filters = convert_to_tuple_vec(&config);
                        cache_filters(&filters);
                        return Ok(filters);
                    }
                    Err(e) => {
                        eprintln!("Failed to parse config file {}: {}", config_path.display(), e);
                        // Fall back to defaults
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read config file {}: {}", config_path.display(), e);
                // Fall back to defaults
            }
        }
    }

    // Use default filters and try to create config file for future edits
    let default_config = default_filters();
    let _ = create_default_config(&default_config, &config_path);

    let filters = convert_to_tuple_vec(&default_config);
    cache_filters(&filters);
    Ok(filters)
}

/// Create default config file if it doesn't exist
fn create_default_config(config: &FileFiltersConfig, path: &PathBuf) -> Result<(), String> {
    // Create directory if it doesn't exist
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }
    }

    // Write default config
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(path, json)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

/// Convert internal format to tuple vector for Python
fn convert_to_tuple_vec(config: &FileFiltersConfig) -> Vec<(String, String)> {
    config
        .filters
        .iter()
        .map(|f| (f.name.clone(), f.extensions.clone()))
        .collect()
}

/// Cache filters in memory
fn cache_filters(filters: &[(String, String)]) {
    if let Ok(mut cache) = FILE_FILTERS_CACHE.lock() {
        *cache = Some(filters.to_vec());
    }
}

/// Reload configuration from disk (useful if user edited the file)
pub fn reload_file_filters() -> Result<Vec<(String, String)>, String> {
    // Clear cache
    if let Ok(mut cache) = FILE_FILTERS_CACHE.lock() {
        *cache = None;
    }
    // Reload from disk
    load_file_filters()
}

/// Get the config directory path (useful for user to locate the file)
pub fn get_config_directory() -> String {
    get_config_dir().to_string_lossy().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_filters_not_empty() {
        let filters = default_filters();
        assert!(!filters.filters.is_empty());
    }

    #[test]
    fn test_tuple_conversion() {
        let config = default_filters();
        let tuples = convert_to_tuple_vec(&config);
        assert_eq!(tuples.len(), config.filters.len());
    }
}


