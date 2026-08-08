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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub name: String,
    pub extensions: String,
}

/// Sort configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortConfig {
    #[serde(default)]
    pub sort_ascending: bool,
    #[serde(default)]
    pub sort_descending: bool,
}

impl Default for SortConfig {
    fn default() -> Self {
        SortConfig {
            sort_ascending: true,
            sort_descending: false,
        }
    }
}

/// Complete configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileFiltersConfig {
    #[serde(default)]
    pub sort: SortConfig,
    pub filters: Vec<FileFilter>,
}

/// Cached configuration
static FILE_FILTERS_CACHE: Lazy<Mutex<Option<Vec<(String, String)>>>> = Lazy::new(|| Mutex::new(None));

/// Get the configuration directory path based on OS
fn get_config_dir() -> PathBuf {
    let name = "icedpygui";
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA")
            .unwrap_or_else(|_| String::from("."));
        PathBuf::from(appdata).join(name)
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| String::from("."));
        PathBuf::from(home).join(format!("Library/Application Support/{}", name))
    }

    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| String::from("."));
        let xdg_config = std::env::var("XDG_CONFIG_HOME").ok();
        
        if let Some(xdg) = xdg_config {
            PathBuf::from(xdg).join(name)
        } else {
            PathBuf::from(home).join(format!(".config/{}", name))
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        PathBuf::from(".").join(format(".{}", name))
    }
}

/// Get the file filters config file path
fn get_config_file_path() -> PathBuf {
    get_config_dir().join("file_filters.json")
}

/// Default file filters
fn default_filters() -> FileFiltersConfig {
    FileFiltersConfig {
        sort: SortConfig {
            sort_ascending: true,
            sort_descending: false,
        },
        filters: vec![
            // Archive Formats
            FileFilter {
                category: Some("Archive".to_string()),
                name: "Archive Files".to_string(),
                extensions: "*.zip;*.rar;*.7z;*.tar;*.gz".to_string(),
            },
            // Audio Formats
            FileFilter {
                category: Some("Audio".to_string()),
                name: "Audio Files".to_string(),
                extensions: "*.mp3;*.wav;*.aac;*.flac;*.ogg".to_string(),
            },
            // Document Formats
            FileFilter {
                category: Some("Documents".to_string()),
                name: "PDF Files".to_string(),
                extensions: "*.pdf".to_string(),
            },
            FileFilter {
                category: Some("Documents".to_string()),
                name: "Text Files".to_string(),
                extensions: "*.txt".to_string(),
            },
            FileFilter {
                category: Some("Documents".to_string()),
                name: "Word Documents".to_string(),
                extensions: "*.docx;*.doc".to_string(),
            },
            // Image Formats
            FileFilter {
                category: Some("Images".to_string()),
                name: "Image Files".to_string(),
                extensions: "*.png;*.jpg;*.jpeg;*.gif;*.bmp;*.svg;*.webp".to_string(),
            },
            FileFilter {
                category: Some("Images".to_string()),
                name: "JPEG Images".to_string(),
                extensions: "*.jpg;*.jpeg".to_string(),
            },
            FileFilter {
                category: Some("Images".to_string()),
                name: "PNG Images".to_string(),
                extensions: "*.png".to_string(),
            },
            // Programming Formats
            FileFilter {
                category: Some("Programming".to_string()),
                name: "C++ Files".to_string(),
                extensions: "*.cpp;*.cc;*.cxx;*.h;*.hpp".to_string(),
            },
            FileFilter {
                category: Some("Programming".to_string()),
                name: "C Files".to_string(),
                extensions: "*.c;*.h".to_string(),
            },
            FileFilter {
                category: Some("Programming".to_string()),
                name: "CSS Files".to_string(),
                extensions: "*.css;*.scss;*.sass;*.less".to_string(),
            },
            FileFilter {
                category: Some("Programming".to_string()),
                name: "HTML Files".to_string(),
                extensions: "*.html;*.htm".to_string(),
            },
            FileFilter {
                category: Some("Programming".to_string()),
                name: "Java Files".to_string(),
                extensions: "*.java".to_string(),
            },
            FileFilter {
                category: Some("Programming".to_string()),
                name: "JavaScript Files".to_string(),
                extensions: "*.js;*.jsx".to_string(),
            },
            FileFilter {
                category: Some("Programming".to_string()),
                name: "JSON Files".to_string(),
                extensions: "*.json".to_string(),
            },
            FileFilter {
                category: Some("Programming".to_string()),
                name: "Python Files".to_string(),
                extensions: "*.py".to_string(),
            },
            FileFilter {
                category: Some("Programming".to_string()),
                name: "Rust Files".to_string(),
                extensions: "*.rs".to_string(),
            },
            FileFilter {
                category: Some("Programming".to_string()),
                name: "TypeScript Files".to_string(),
                extensions: "*.ts;*.tsx".to_string(),
            },
            FileFilter {
                category: Some("Programming".to_string()),
                name: "XML Files".to_string(),
                extensions: "*.xml".to_string(),
            },
            FileFilter {
                category: Some("Programming".to_string()),
                name: "YAML Files".to_string(),
                extensions: "*.yaml;*.yml".to_string(),
            },
            // Video Formats
            FileFilter {
                category: Some("Video".to_string()),
                name: "Video Files".to_string(),
                extensions: "*.mp4;*.avi;*.mkv;*.mov;*.flv;*.wmv".to_string(),
            },
            // All Files
            FileFilter {
                category: None,
                name: "All Files".to_string(),
                extensions: "*.*".to_string(),
            },
        ],
    }
}

/// Load filters from disk or create default config
pub fn load_file_filters() -> Result<Vec<(String, String)>, String> {
    // Check cache first
    if let Ok(cache) = FILE_FILTERS_CACHE.lock()
        && let Some(filters) = cache.as_ref() {
            return Ok(filters.clone());
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
    if let Some(parent) = path.parent()
        && !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
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
    let mut filters: Vec<(String, String)> = config
        .filters
        .iter()
        .map(|f| (f.name.clone(), f.extensions.clone()))
        .collect();
    
    // Apply sorting based on configuration
    if config.sort.sort_ascending {
        filters.sort_by(|a, b| a.0.cmp(&b.0));
    } else if config.sort.sort_descending {
        filters.sort_by(|a, b| b.0.cmp(&a.0));
    }
    
    filters
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


