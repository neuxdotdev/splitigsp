```
.  # src
├── error.rs
├── lib
│   ├── input_output_pdf.rs
│   ├── mod.rs
│   ├── rename_pdf.rs
│   ├── split_pdf.rs
├── main.rs
├── types.rs
├── ui.rs
```

## src/main.rs

```rust
#[allow(special_module_name)] // <- Hilangkan warning "lib"
mod lib;
mod error;
mod types;
mod ui;

pub fn main() -> iced::Result {
    // Iced 0.14 wajib pakai format ini: || (State, Task)
    iced::application(
        || (ui::App::default(), iced::Task::none()),
        ui::App::update,
        ui::App::view,
    )
    .title("Slitigs-SP")
    .theme(|_| iced::Theme::Dark)
    .run()
}
```

## src/lib/mod.rs

```rust
pub mod input_output_pdf;
pub mod rename_pdf;
pub mod split_pdf;
```

## src/lib/split_pdf.rs

```rust
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
```

## src/lib/rename_pdf.rs

```rust
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
```

## src/lib/input_output_pdf.rs

```rust
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
```

## src/error.rs

```rust
#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Lopdf(lopdf::Error),
    Regex(regex::Error),
    Custom(String),
}
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}
impl From<lopdf::Error> for AppError {
    fn from(e: lopdf::Error) -> Self {
        AppError::Lopdf(e)
    }
}
impl From<regex::Error> for AppError {
    fn from(e: regex::Error) -> Self {
        AppError::Regex(e)
    }
}
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO Error: {}", e),
            AppError::Lopdf(e) => write!(f, "PDF Error: {}", e),
            AppError::Regex(e) => write!(f, "Regex Error: {}", e),
            AppError::Custom(e) => write!(f, "{}", e),
        }
    }
}
```

## src/types.rs

```rust
#[derive(Debug, Clone)]
pub struct PdfData {
    pub ref_no: String,
    pub nama: String,
    pub perihal: String,
}
```

## src/ui.rs

```rust
use iced::widget::{button, column, text, text_input};
use iced::{Element, Task};
use std::path::PathBuf;
use crate::lib::split_pdf;
use crate::error::AppError;
pub struct App {
    file_path: String,
    status: String,
    is_processing: bool,
}
impl Default for App {
    fn default() -> Self {
        Self {
            file_path: String::new(),
            status: "Pilih file PDF untuk mulai split...".to_string(),
            is_processing: false,
        }
    }
}
#[derive(Debug)]
pub enum Message {
    SelectFile,
    StartProcess,
    ProcessFinished(Result<String, AppError>),
}
impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectFile => {
                let file = rfd::FileDialog::new()
                    .add_filter("PDF", &["pdf"])
                    .pick_file();
                if let Some(path_buf) = file {
                    self.file_path = path_buf.display().to_string();
                    self.status = "File siap diproses. Klik PROSES.".to_string();
                }
                Task::none()
            }
            Message::StartProcess => {
                if self.file_path.is_empty() {
                    self.status = "Error: Silakan pilih file dulu!".to_string();
                    return Task::none();
                }
                self.is_processing = true;
                self.status = "Sedang memproses... Mohon tunggu.".to_string();
                let path = PathBuf::from(&self.file_path);
                Task::perform(
                    async move { split_pdf::process_split_pdf(&path) },
                    Message::ProcessFinished,
                )
            }
            Message::ProcessFinished(result) => {
                self.is_processing = false;
                match result {
                    Ok(msg) => self.status = msg,
                    Err(e) => self.status = format!("Error: {}", e),
                }
                Task::none()
            }
        }
    }
    pub fn view(&self) -> Element<Message> {
        let file_input = text_input("Path PDF...", &self.file_path);
        let btn_select = button("Pilih File PDF").on_press(Message::SelectFile);
        let btn_process = if self.is_processing {
            button("Memproses...")
        } else {
            button("PROSES & SPLIT!").on_press(Message::StartProcess)
        };
        let status_text = text(&self.status);
        column![btn_select, file_input, btn_process, status_text]
            .padding(20)
            .spacing(15)
            .into()
    }
}
```
