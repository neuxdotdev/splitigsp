use lopdf::Document;
use std::path::Path;
use crate::error::AppError;
use crate::lib::input_output_pdf::get_output_dir;
use crate::lib::rename_pdf::{extract_pdf_data, generate_filename};
pub fn process_split_pdf(input_path: &Path) -> Result<String, AppError> {
    let doc = Document::load(input_path)?;
    let out_dir = get_output_dir()?;
    let pages = doc.get_pages();
    let total_pages = pages.len();
    if total_pages == 0 {
        return Err(AppError::Custom("PDF kosong, tidak ada halaman.".to_string()));
    }
    for (page_num, _page_id) in pages.clone().into_iter() {
        let text = doc.extract_text(&[page_num]).unwrap_or_default();
        let pdf_data = extract_pdf_data(&text);
        let filename = generate_filename(&pdf_data, page_num);
        let output_path = out_dir.join(&filename);
        let mut single_page_doc = doc.clone();
        let pages_to_delete: Vec<u32> = single_page_doc
            .get_pages()
            .keys()
            .filter(|&&p| p != page_num)
            .cloned()
            .collect();
        for p in pages_to_delete {
            single_page_doc.delete_pages(&[p]);
        }
        single_page_doc.save(&output_path)?;
    }
    Ok(format!("Sukses! {} halaman diproses ke:\n{}", total_pages, out_dir.display()))
}