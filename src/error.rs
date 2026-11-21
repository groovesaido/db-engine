use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    PageNotFound,
}
pub type Result<T> = std::result::Result<T, Error>;
