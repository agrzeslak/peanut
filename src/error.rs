use thiserror::Error;

#[non_exhaustive]
#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("multiple matching instructions were found: {0}")]
    AmbiguousInstruction(String),
    #[error("could not convert type: {0}")]
    CannotCovertType(String),
    #[error("instruction could not be parsed: {0}")]
    CannotParseInstruction(String),
    #[error("invalid effective address: {0}")]
    InvalidEffectiveAddress(String),
    #[error("inaccessible address: {0}")]
    InaccessibleAddress(String),
    #[error("invalid operand type: {0}")]
    InvalidOperandType(String),
    #[error("no matching instruction could be found: {0}")]
    NoMatchingInstruction(String),
}
