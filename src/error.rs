#[derive(Error, Debug)]
pub enum ConsumerError {
    #[cfg(feature = "hyper-client")]
    #[error("Error from Hyper: {0}")]
    HyperError(#[from] hyper::Error),

    #[cfg(feature = "isahc-client")]
    #[error("Error from Isahc: {0}")]
    IsahcError(#[from] isahc::Error),

    #[error("Error from Http: {0}")]
    HttpError(#[from] http::Error),

    #[error("Unknown error")]
    Unknown,
}
