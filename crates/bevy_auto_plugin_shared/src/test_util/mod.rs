#[cfg(test)]
pub mod combo;
pub(crate) mod macros;

pub(crate) fn assert_tokens_match(
    plugin: impl std::fmt::Debug,
    input: impl ToString,
    args: impl quote::ToTokens,
) {
    let input = input.to_string();
    assert_eq!(
        args.to_token_stream().to_string(),
        input,
        concat!(
            "failed to expand into expected tokens - args: ",
            stringify!($args_ident),
            ", plugin: {:?}, args_inner: {}"
        ),
        plugin,
        input,
    );
}
