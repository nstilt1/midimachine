#[derive(Debug)]
pub struct MusicError {
    pub message: String
}

impl From<&str> for MusicError {
    /**
     * Allows creating the default error with just the error message.
     */
    fn from(error: &str) -> Self {
        MusicError { message: error.to_string() }
    }
}

impl From<String> for MusicError {
    /**
     * Allows creating the default error with just the error message.
     */
    fn from(error: String) -> Self {
        MusicError { message: error }
    }
}