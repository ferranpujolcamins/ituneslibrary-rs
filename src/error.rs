use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Couldn't initialize the iTunes library")]
    CouldNotCreateLibrary
}
