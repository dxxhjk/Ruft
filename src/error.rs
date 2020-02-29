use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum InitializationError {
    NodeInitializationError,
    RPCInitializationError,
    ClientServerInitializationError,
}

impl fmt::Display for InitializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InitializationError::NodeInitializationError => write!(f, "Initializing Node Error"),
            InitializationError::RPCInitializationError => write!(f, "Initializing RPC Error"),
            InitializationError::ClientServerInitializationError => write!(f, "Initializing ClientServer Error"),
        }
    }
}

impl Error for InitializationError {}
