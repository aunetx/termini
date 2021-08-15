use gtk::glib;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    GLibError(glib::Error),
    GLibBoolError(glib::BoolError),
    IOError(std::io::Error),
}

impl From<glib::Error> for Error {
    fn from(e: glib::Error) -> Self {
        Self::GLibError(e)
    }
}

impl From<glib::BoolError> for Error {
    fn from(e: glib::BoolError) -> Self {
        Self::GLibBoolError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}
