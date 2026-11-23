#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

pub struct SecurityEvent {
    pub timestamp: String,
    pub severity: Severity,
    pub ip: String,
    pub username: String,
    pub action: String,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidColumnCount,
    UnkownSeverityy(String),
    EmptyField,
}

