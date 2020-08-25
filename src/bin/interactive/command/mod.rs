use bookmark_lib::Registry;
use crate::interactive::table::StatefulTable;
use crate::interactive::url_table_item::{URLItem, URLItemSource};
use tui::backend::Backend;
use crate::interactive::modules::{HandleInput, Draw};
use std::fmt::{Debug, Display};
use serde::export::Formatter;

pub mod tag;

pub trait Command<R: Registry, B: Backend>: HandleInput<R> + Draw<B> + Execute<R> {}

pub trait Execute<R: Registry> {
    fn execute(&self, args: Vec<&str>) -> Result<bool, Error>; // TODO: consider what should be returned here
}

// TODO: Move it further to the top?

#[derive(Debug)]
pub enum ErrorType {
    InputError,
    InternalError,
}

#[derive(Debug)]
pub struct Error {
    error: String,
    error_type: ErrorType,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(self.error.as_str())
    }
}

impl std::error::Error for Error {}

impl Error {

    pub fn new_input(error: &str) -> Error {
        Error {
            error: error.to_string(),
            error_type: ErrorType::InputError,
        }
    }

    pub fn new_internal(error: String) -> Error {
        Error {
            error,
            error_type: ErrorType::InternalError,
        }
    }

}

impl std::convert::From<std::boxed::Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Error::new_internal(err.to_string())
    }
}
