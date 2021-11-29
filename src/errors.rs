pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DBOpenError,
    DBError,
    TaskCreationError,
    MissingTask,
}

impl From<rusqlite::Error> for Error {
    fn from(_: rusqlite::Error) -> Self {
        Error::DBError
    }
}
