mod consumer;

#[cfg(feature = "hyper-client")]
mod hyper;
#[cfg(feature = "hyper-client")]
pub use self::hyper::HyperClient;

#[cfg(feature = "isahc-client")]
mod isahc;
#[cfg(feature = "isahc-client")]
pub use ::isahc::HttpClient as IsahcClient;

pub use self::consumer::Consumer;
pub use self::consumer::ConsumerTrait;
