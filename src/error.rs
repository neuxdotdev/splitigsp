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
