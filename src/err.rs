use std::fmt;

#[derive(Debug,Clone)]
pub enum Error{
    EncodeJsonError(String),
    ParseJsonError(String),
}

impl Error{
    fn as_str(&self) -> &str{
        match *self{
            Error::ParseJsonError(ref err) => err,
            Error::EncodeJsonError(ref err) => err,
        }
    }
}

// impl display for Error
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// impl std::error::Error for Error
impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.as_str()
    }
}