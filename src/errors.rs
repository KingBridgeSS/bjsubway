#[derive(thiserror::Error, Debug)]
pub enum NetError {
    #[error("ReqwestError")]
    ReqwestError(#[from] reqwest::Error),
    #[error("IOError")]
    IOError(#[from] std::io::Error),
}