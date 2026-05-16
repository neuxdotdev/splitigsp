use crate::types::PdfData;
use regex::Regex;
pub fn extract_pdf_data(text: &str) -> PdfData {
    let mut ref_no = "UNKNOWN_REF".to_string();
    let mut nama = "UNKNOWN_NAMA".to_string();
    let mut perihal = "SURAT".to_string();
    if let Ok(re) = Regex::new(r"Ref No\s*:\s*([A-Z0-9]+)") {
        if let Some(caps) = re.captures(text) {
            ref_no = caps[1].trim().to_string();
        }
    }
    if let Ok(re) = Regex::new(r"Bapak/Ibu\s+(.*?)(?:\r?\n)") {
        if let Some(caps) = re.captures(text) {
            nama = caps[1].trim().to_string();
        }
    }
    if let Ok(re) = Regex::new(r"Perihal\s*:\s*(.*?)(?:\r?\n)") {
        if let Some(caps) = re.captures(text) {
            perihal = caps[1].trim().to_string();
        }
    }
    PdfData {
        ref_no,
        nama,
        perihal,
    }
}
pub fn generate_filename(data: &PdfData, page_num: u32) -> String {
    let raw_filename = if data.ref_no == "UNKNOWN_REF" && data.nama == "UNKNOWN_NAMA" {
        format!("Halaman_{}.pdf", page_num)
    } else {
        format!("{} - {} {}.pdf", data.ref_no, data.perihal, data.nama)
    };
    let safe_filename: String = raw_filename
        .chars()
        .map(|c| match c {
            '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect();
    safe_filename
}
