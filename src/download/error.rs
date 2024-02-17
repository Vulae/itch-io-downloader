
use core::fmt;
use std::error::Error;



#[derive(Debug, Clone)]
pub enum DownloadError {
    LibraryFailLoad,
    LibraryGameIdMismatch,
    GameNoExecutable,
    ExtractFailed(String)
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DownloadError::LibraryFailLoad => write!(f, "Library failed loading."),
            DownloadError::LibraryGameIdMismatch => write!(f, "Game ID Mismatch."),
            DownloadError::GameNoExecutable => write!(f, "Game failed to find executable."),
            DownloadError::ExtractFailed(msg) => write!(f, "Extraction failed {}", msg),
        }
    }
}

impl Error for DownloadError { }


