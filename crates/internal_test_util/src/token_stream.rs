#![allow(unused)]

use proc_macro2::TokenStream;
use quote::ToTokens;
use std::fmt::{
    Debug,
    Formatter,
};

#[inline]
pub fn token_string(ts: impl ToTokens) -> String {
    ts.to_token_stream().to_string()
}

pub struct Ts(TokenStream);

impl Debug for Ts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl PartialEq for Ts {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string() == other.0.to_string()
    }
}

pub fn ts(token_stream: impl ToTokens) -> Ts {
    Ts(token_stream.to_token_stream())
}

impl<T> From<T> for Ts
where
    T: ToTokens,
{
    fn from(value: T) -> Self {
        Self(value.into_token_stream())
    }
}

#[macro_export]
macro_rules! assert_ts_eq {
    ($left:expr, $right:expr $(, $msg:expr)?) => {
        assert_eq!($crate::token_stream::ts($left), $crate::token_stream::ts($right), $($msg)?);
    };
}
