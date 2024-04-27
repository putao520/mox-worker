use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub(crate) no: i32,
    pub(crate) msg: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GscError {{ no: {}, msg: {} }}", self.no, self.msg)
    }
}

impl std::error::Error for Error {}