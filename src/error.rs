#[derive(Error, Debug)]
pub enum ConsumerError {
    #[cfg(feature = "hyper-client")]
    #[error("Error from Hyper: {0}")]
    HyperError(#[from] hyper::Error),

    #[error("Unknown error")]
    Unknown,
}
