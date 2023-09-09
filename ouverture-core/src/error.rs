use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("message too big (maximum size is 64k)")]
    MessageTooBig,
    #[error("not native protocol")]
    NotNativeProtocol,
    #[error("unknown server error")]
    Unknown,
}
