use std::array::TryFromSliceError;
use std::error::Error;
use std::fmt;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum DoomError {
  Number(TryFromSliceError),
  String(FromUtf8Error),
  Wad(String),
}

impl fmt::Display for DoomError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      DoomError::Number(ref cause) => write!(f, "Convert error: {}", cause),
      DoomError::String(ref cause) => write!(f, "String error: {}", cause),
      DoomError::Wad(ref cause) => write!(f, "Wad error: {}", cause),
    }
  }
}

impl Error for DoomError {
  fn description(&self) -> &str {
    match *self {
      DoomError::Number(ref cause) => cause.description(),
      DoomError::String(ref cause) => cause.description(),
      DoomError::Wad(ref cause) => cause.as_str(),
    }
  }
  fn cause(&self) -> Option<&dyn Error> {
    match &self {
      DoomError::Number(ref cause) => Some(cause),
      DoomError::String(ref cause) => Some(cause),
      DoomError::Wad(ref _cause) => None,
    }
  }
}

impl From<TryFromSliceError> for DoomError {
  fn from(cause: TryFromSliceError) -> Self {
    DoomError::Number(cause)
  }
}

impl From<FromUtf8Error> for DoomError {
  fn from(cause: FromUtf8Error) -> Self {
    DoomError::String(cause)
  }
}
