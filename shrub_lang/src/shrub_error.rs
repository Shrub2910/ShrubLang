#[derive(Debug)]
pub struct SyntaxError {
    pub message: String,
    pub line_number: usize,
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
