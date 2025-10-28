#![allow(unused)]

use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug)]
pub struct Ts(TokenStream);

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
