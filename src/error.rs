use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    #[error("instruction could not be parsed")]
    InvalidInstruction,
}
