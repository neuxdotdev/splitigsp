
use iced::widget::{button, column, text, text_input};
use iced::{Element, Task};
use std::path::PathBuf;
use crate::lib::split_pdf;
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
#[derive(Debug, Clone)]             
pub enum Message {
    SelectFile,
    StartProcess,
    ProcessFinished(Result<String, String>),  
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
                    async move {
                        split_pdf::process_split_pdf(&path)
                            .map_err(|e| e.to_string())   
                    },
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