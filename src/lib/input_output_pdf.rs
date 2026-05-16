use chrono::Local;
use std::path::PathBuf;
pub fn get_output_dir() -> Result<PathBuf, crate::error::AppError> {
    let mut doc_dir = dirs::document_dir().ok_or_else(|| {
        crate::error::AppError::Custom("Folder Documents tidak ditemukan".to_string())
    })?;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    doc_dir.push("splitigsp");
    doc_dir.push(&timestamp);
    std::fs::create_dir_all(&doc_dir)?;
    Ok(doc_dir)
}
