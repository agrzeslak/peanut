use thiserror::Error;

#[non_exhaustive]
#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("instruction could not be parsed: {0}")]
    CannotParseInstruction(String),
    #[error("could not convert type: {0}")]
    CannotCovertType(String),
}
