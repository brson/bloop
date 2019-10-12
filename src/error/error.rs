use std::error::Error as StdError;
use std::result::Result as StdResult;
use std::fmt::{self, Display, Debug, Formatter};

pub type BResult<T> = StdResult<T, BError>;

pub struct BError(Box<BErrorInner>);

// FIXME: Figure out how to use unsized types for unboxed kind
// FIXME: Limit kind to all types that implement Display
// _except_ for str / String
// FIXME: Would like to make it possible to require
// context be added to errors before returning.
struct BErrorInner {
    kind: Box<dyn Display + Send + Sync + 'static>,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl BError {
    pub fn new<K>(kind: K) -> BError
    where K: Display + Send + Sync + 'static
    {
        BError(Box::new(BErrorInner {
            kind: Box::new(kind),
            source: None,
        }))
    }

    pub fn from_source<E, K>(source: E, kind: K) -> BError
    where E: StdError + Send + Sync + 'static,
          K: Display + Send + Sync + 'static
    {
        BError(Box::new(BErrorInner {
            kind: Box::new(kind),
            source: Some(Box::new(source)),
        }))
    }
}

impl StdError for BError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self.0.source {
            Some(ref e) => Some(e.as_ref()),
            None => None,
        }
    }
}

impl Display for BError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.kind.fmt(f)
    }
}

impl Debug for BError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("BError")
            .field("kind", &format!("{}", &self.0.kind))
            .field("source", &self.0.source)
            .finish()
    }
}

pub trait ResultExt<T> {
    fn ec<K>(self, kind: K) -> StdResult<T, BError>
    where K: Display + Send + Sync + 'static;
}

impl<T, E> ResultExt<T> for StdResult<T, E>
where E: StdError + Send + Sync + 'static
{
    fn ec<K>(self, kind: K) -> StdResult<T, BError>
    where K: Display + Send + Sync + 'static
    {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(BError::from_source(e, kind))
        }
    }
}

#[test]
fn test() {
    #[derive(Debug)]
    pub struct TestError;

    impl StdError for TestError { }

    impl Display for TestError {
        fn fmt(&self, _f: &mut Formatter) -> fmt::Result {
            panic!()
        }
    }

    fn ret_s_t() -> StdResult<(), TestError> { panic!() }
    fn ret_s_b() -> StdResult<(), BError> { panic!() }

    fn try_s_t() -> StdResult<(), TestError> {
        //ret_s_t().c("test")?;
        //ret_s_t().c("test")?;

        Ok(())
    }

    fn try_s_b() -> StdResult<(), BError> {
        ret_s_t().c("test")?;
        ret_s_b().c("test")?;

        Ok(())
    }

}
