use crate::task::types::StartedTask;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    DBOpenError,
    DBError,
    TaskCreationError,
    MissingTask,
    AlreadyOpenedTask(StartedTask),
}

impl From<rusqlite::Error> for Error {
    fn from(_: rusqlite::Error) -> Self {
        Error::DBError
    }
}

pub fn display_error(err: Error) {
    match err {
        Error::AlreadyOpenedTask(task) => println!(
            "{}\nThere is an already opened task. Finish the current task before opening another one.", task
        ),
        _ => println!("Ups there was an error."),
    }
}
