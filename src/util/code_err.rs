use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CodeParseError {
    EarlyEOF(String),
    StaticAnalysisTypeMismatch {
        expected: String,
        got: String,
        for_what: String,
    },
    ImprobableCast {
        expected: String,
        got: String,
        where_: String,
    },
    InvalidFormat,
}
#[derive(Debug)]
pub enum ClassParseError {
    EarlyEOF(String),
    BadValue {
        expected: String,
        got: String,
        for_what: String,
    },
    CodeParseError {
        internal: CodeParseError,
        classpath: String,
        signature: Option<String>,
    },
    InvalidClassfileVersion {
        major: u16,
        minor: u16,
        too_new: bool,
        supported_version_max: (u16, u16),
        supported_version_min: (u16, u16),
    },
    IOError {
        internal: std::io::Error,
        classpath: Option<String>,
        path: Option<String>
    },
    StringDecodeError {
        internal: std::string::FromUtf8Error,
        buffer: Vec<u8>,
    },
    Silly(String),
}
impl Display for ClassParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for ClassParseError {}
