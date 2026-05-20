use std::{borrow::Cow, io::Read};

use bytes::Bytes;

#[derive(Default)]
pub enum Body {
    #[default]
    Empty,
    Full(Bytes),
    Stream(Box<dyn Read>),
}

impl Body {
    pub fn empty() -> Self {
        Self::Empty
    }

    pub fn full(bytes: impl Into<Bytes>) -> Self {
        Self::Full(bytes.into())
    }

    pub fn stream(bytes: impl Read + 'static) -> Self {
        Self::Stream(Box::new(bytes))
    }
}

impl<'a> From<Cow<'a, str>> for Body {
    fn from(value: Cow<'a, str>) -> Self {
        Self::Full(value.into_owned().into())
    }
}

impl From<&str> for Body {
    fn from(value: &str) -> Self {
        Cow::Borrowed(value).into()
    }
}

impl From<String> for Body {
    fn from(value: String) -> Self {
        Cow::<'_, str>::Owned(value).into()
    }
}

impl From<Cow<'_, [u8]>> for Body {
    fn from(value: Cow<'_, [u8]>) -> Self {
        Self::Full(value.into_owned().into())
    }
}

impl From<&[u8]> for Body {
    fn from(value: &[u8]) -> Self {
        Cow::Borrowed(value).into()
    }
}

impl From<Vec<u8>> for Body {
    fn from(value: Vec<u8>) -> Self {
        Cow::<'_, [u8]>::Owned(value).into()
    }
}

impl From<Bytes> for Body {
    fn from(value: Bytes) -> Self {
        Self::Full(value)
    }
}
