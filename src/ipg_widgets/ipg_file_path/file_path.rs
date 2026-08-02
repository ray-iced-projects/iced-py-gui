//! File path selection utility for opening system file/folder dialogs

use rfd::FileDialog;

/// Opens a system file or folder picker dialog
///
/// # Arguments
/// * `select_file` - If true, opens file picker
/// * `select_folder` - If true, opens folder picker
///
/// # Returns
/// `Some(path)` if user selected a file/folder, `None` if cancelled
///
/// # Example
/// ```
/// let path = pick_path(Some(true), None);  // File picker
/// let folder = pick_path(None, Some(true));  // Folder picker
/// ```
pub fn pick_path(select_file: Option<bool>, select_folder: Option<bool>) -> Option<String> {
    // Determine which picker to use
    let is_file = select_file.unwrap_or(false);
    let is_folder = select_folder.unwrap_or(false);

    if is_file && !is_folder {
        // File picker
        FileDialog::new()
            .pick_file()
            .map(|path| path.to_string_lossy().to_string())
    } else if is_folder && !is_file {
        // Folder picker
        FileDialog::new()
            .pick_folder()
            .map(|path| path.to_string_lossy().to_string())
    } else {
        // Invalid: both or neither specified
        None
    }
}

/// Opens a system file picker with optional file type filter
///
/// # Arguments
/// * `extensions` - Optional list of file extensions (e.g., vec!["json", "txt"])
///
/// # Returns
/// `Some(path)` if user selected a file, `None` if cancelled
pub fn pick_file_with_filter(extensions: Option<Vec<&str>>) -> Option<String> {
    let mut dialog = FileDialog::new();

    if let Some(exts) = extensions {
        dialog = dialog.add_filter("Filtered Files", &exts);
    }

    dialog
        .pick_file()
        .map(|path| path.to_string_lossy().to_string())
}

/// Opens a system folder picker
///
/// # Returns
/// `Some(path)` if user selected a folder, `None` if cancelled
pub fn pick_folder() -> Option<String> {
    FileDialog::new()
        .pick_folder()
        .map(|path| path.to_string_lossy().to_string())
}

/// Opens a system file picker
///
/// # Returns
/// `Some(path)` if user selected a file, `None` if cancelled
pub fn pick_file() -> Option<String> {
    FileDialog::new()
        .pick_file()
        .map(|path| path.to_string_lossy().to_string())
}

