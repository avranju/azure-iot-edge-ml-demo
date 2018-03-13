use std::convert::From;
use std::error::Error as StdError;
use std::fmt;
use std::fmt::Display;

use failure::{Backtrace, Context, Fail};

use ws::{Error as WsError, ErrorKind as WsErrorKind};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Gstreamer element missing: {}", _0)]
    MissingElement(&'static str),

    #[fail(display = "System error orccurred: {}", _0)]
    SystemError(Box<StdError + Send + Sync + 'static>),
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<Error> for WsError {
    fn from(err: Error) -> WsError {
        WsError::new(
            WsErrorKind::Custom(Box::new(err.compat())),
            "Image server error",
        )
    }
}
