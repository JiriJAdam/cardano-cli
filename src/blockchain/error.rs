use std::{io, fmt, error};
use cardano::block::{self, HeaderHash};
use cardano_storage;
use cbor_event;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),

    NewCannotInitializeBlockchainDirectory(cardano_storage::Error),

    ListNoBlockchains,
    ListPermissionsDenied,
    ListBlockchainInvalidName(::storage_units::utils::directory_name::DirectoryNameError),

    ForwardHashDoesNotExist(HeaderHash),

    GetBlockDoesNotExist(HeaderHash),
    GetInvalidBLock(HeaderHash),

    CatMalformedBlock(cbor_event::Error),

    VerifyInvalidBlock(block::Error),
    VerifyMalformedBlock(cbor_event::Error),

    VerifyChainGenesisHashNotFound(HeaderHash),
    VerifyChainInvalidGenesisPrevHash(HeaderHash, HeaderHash), // (Expected, got)
    BlockchainIsNotValid(usize),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self { Error::IoError(e) }
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError(_) => write!(f, "I/O Error"),

            Error::NewCannotInitializeBlockchainDirectory(_) => write!(f, "Cannot Initialise the blockchain directory"),
            Error::ListNoBlockchains => write!(f, "No local blockchains yet"),
            _ => unimplemented!()
        }
    }
}
impl error::Error for Error {
    fn cause(&self) -> Option<& error::Error> {
        match self {
            Error::IoError(ref err) => Some(err),
            _ => unimplemented!()
        }
    }
}