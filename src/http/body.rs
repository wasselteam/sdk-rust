use std::{borrow::Cow, io::Read};

#[derive(Default)]
pub enum Body {
    #[default]
    Empty,
    Full(Vec<u8>),
    Stream(Box<dyn Read>),
}

impl Read for Body {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Body::Empty => Ok(0),
            Body::Full(bytes) => {
                let amt = std::cmp::min(buf.len(), bytes.len());
                buf[..amt].copy_from_slice(&bytes[..amt]);
                *bytes = bytes.split_off(amt);
                Ok(amt)
            }
            Body::Stream(read) => read.read(buf),
        }
    }
}

impl<'a> From<Cow<'a, str>> for Body {
    fn from(value: Cow<'a, str>) -> Self {
        Self::Full(value.as_bytes().to_vec())
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
        Self::Full(value.to_vec())
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
