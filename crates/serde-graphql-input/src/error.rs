use std::{fmt::Display, io, result};
pub struct Error {
    err: Box<ErrorImpl>,
}

impl Error {
    pub(crate) fn io(error: io::Error) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Io(error),
            }),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

struct ErrorImpl {
    code: ErrorCode,
}

pub(crate) enum ErrorCode {
    Message(Box<str>),
    Io(io::Error),
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Message(msg.to_string().into_boxed_str()),
            }),
        }
    }
}

impl serde::ser::StdError for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error({:?})", self.err.code.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&*self.err, f)
    }
}

impl Display for ErrorImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::Message(m) => f.write_str(m),
            ErrorCode::Io(err) => Display::fmt(err, f),
        }
    }
}
