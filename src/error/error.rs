pub use failure::ResultExt;
pub type BError = failure::Error;
pub type BResult<T> = Result<T, BError>;
