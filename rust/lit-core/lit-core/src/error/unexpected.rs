#[cfg(feature = "chrono")]
use chrono::LocalResult;
use std::panic::Location;
use std::sync::Arc;

use crate::error::{err_pkg_name, Code, Error, Kind, Result};

// TODO: Currently this will only use the lit_core PKG_NAME.
pub trait Unexpected<T> {
    fn expect_or_err<M>(self, msg: M) -> Result<T>
    where
        M: AsRef<str> + Into<String>;
    fn expect_or_err_code<C, M>(self, code: C, msg: M) -> Result<T>
    where
        M: AsRef<str> + Into<String>,
        C: Code + Send + Sync + 'static;
}

impl<T> Unexpected<T> for Option<T> {
    #[track_caller]
    #[inline]
    fn expect_or_err<M>(self, msg: M) -> Result<T>
    where
        M: AsRef<str> + Into<String>,
    {
        match self {
            Some(v) => Ok(v),
            None => Err(Error::new(
                Some(Kind::Unexpected),
                err_pkg_name(),
                Some(msg.into()),
                None,
                Some("unexpected empty Option"),
                Some(Location::caller()),
            )),
        }
    }

    #[track_caller]
    #[inline]
    fn expect_or_err_code<C, M>(self, code: C, msg: M) -> Result<T>
    where
        M: AsRef<str> + Into<String>,
        C: Code + Send + Sync + 'static,
    {
        match self {
            Some(v) => Ok(v),
            None => Err(Error::new(
                Some(Kind::Unexpected),
                err_pkg_name(),
                Some(msg.into()),
                Some(Arc::new(code)),
                Some("unexpected empty Option"),
                Some(Location::caller()),
            )),
        }
    }
}

impl<T, E> Unexpected<T> for std::result::Result<T, E>
where
    E: std::error::Error + Sync + Send + 'static,
{
    #[track_caller]
    #[inline]
    fn expect_or_err<M>(self, msg: M) -> Result<T>
    where
        M: AsRef<str> + Into<String>,
    {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                return Err(Error::new(
                    Some(Kind::Unexpected),
                    err_pkg_name(),
                    Some(format!("unexpected err in Result: {}", msg.as_ref())),
                    None,
                    Some(e),
                    Some(Location::caller()),
                ));
            }
        }
    }

    #[track_caller]
    #[inline]
    fn expect_or_err_code<C, M>(self, code: C, msg: M) -> Result<T>
    where
        M: AsRef<str> + Into<String>,
        C: Code + Send + Sync + 'static,
    {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                return Err(Error::new(
                    Some(Kind::Unexpected),
                    err_pkg_name(),
                    Some(format!("unexpected err in Result: {}", msg.as_ref())),
                    Some(Arc::new(code)),
                    Some(e),
                    Some(Location::caller()),
                ));
            }
        }
    }
}

#[cfg(feature = "chrono")]
impl<T> Unexpected<T> for LocalResult<T> {
    #[track_caller]
    #[inline]
    fn expect_or_err<M>(self, msg: M) -> Result<T>
    where
        M: AsRef<str> + Into<String>,
    {
        match self {
            LocalResult::Single(v) => Ok(v),
            LocalResult::None => {
                return Err(Error::new(
                    Some(Kind::Unexpected),
                    err_pkg_name(),
                    Some(msg.into()),
                    None,
                    Some("unexpected err in chrono LocalResult: None/invalid"),
                    Some(Location::caller()),
                ));
            }
            LocalResult::Ambiguous(_, _) => {
                return Err(Error::new(
                    Some(Kind::Unexpected),
                    err_pkg_name(),
                    Some(msg.into()),
                    None,
                    Some("unexpected err in chrono LocalResult: Ambiguous/invalid"),
                    Some(Location::caller()),
                ));
            }
        }
    }

    #[track_caller]
    #[inline]
    fn expect_or_err_code<C, M>(self, code: C, msg: M) -> Result<T>
    where
        M: AsRef<str> + Into<String>,
        C: Code + Send + Sync + 'static,
    {
        match self {
            LocalResult::Single(v) => Ok(v),
            LocalResult::None => {
                return Err(Error::new(
                    Some(Kind::Unexpected),
                    err_pkg_name(),
                    Some(msg.into()),
                    Some(Arc::new(code)),
                    Some("unexpected err in chrono LocalResult: None/invalid"),
                    Some(Location::caller()),
                ));
            }
            LocalResult::Ambiguous(_, _) => {
                return Err(Error::new(
                    Some(Kind::Unexpected),
                    err_pkg_name(),
                    Some(msg.into()),
                    Some(Arc::new(code)),
                    Some("unexpected err in chrono LocalResult: Ambiguous/invalid"),
                    Some(Location::caller()),
                ));
            }
        }
    }
}
