use std::{fmt::write, io};

use crate::pager::PageNo;

#[derive(Debug)]
pub enum Error {
    PageOutOfRange(PageNo),
    DoubleFree(PageNo),
}
pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PageOutOfRange(n) => write!(f, "page {} out of range", n),
            Error::DoubleFree(n) => write!(f, "page {} double-freed", n),
        }
    }
}
impl std::error::Error for Error {}
