use dungeon::{X, Y};
use error_chain_mini::ChainedError;
pub(crate) use error_chain_mini::ErrorKind;
pub(crate) use error_chain_mini::ResultExt;
use input::Key;
use rect_iter::IndexError;
/// Our own ErrorKind type
#[derive(Clone, Debug, ErrorKind)]
pub enum ErrorId {
    #[msg(short = "Invalid index access", detailed = "{:?}", _0)]
    Index(IndexError),
    #[msg(short = "Invalid input", detailed = "key: {:?}", _0)]
    Input(Key),
    #[msg(short = "Incomplete input")]
    IncompleteInput,
    // it's intended to use only in 'immediate panic pattern'
    #[msg(short = "Logic error")]
    LogicError,
}

impl From<IndexError> for ErrorId {
    fn from(e: IndexError) -> Self {
        ErrorId::Index(e)
    }
}

pub type GameError = ChainedError<ErrorId>;

pub type GameResult<T> = Result<T, GameError>;