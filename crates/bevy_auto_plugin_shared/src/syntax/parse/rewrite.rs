use proc_macro2::TokenStream;
#[cfg(feature = "compat_generics_angles")]
use proc_macro2::{
    Delimiter,
    Group,
    Punct,
    Spacing,
    TokenTree,
};

/// Rewrite `field_name = <...>` to `field_name(...)`.
#[cfg(feature = "compat_generics_angles")]
pub fn rewrite_generics_angles(tokens: TokenStream, field_name: &str) -> TokenStream {
    fn is_lt(p: &Punct) -> bool {
        p.as_char() == '<'
    }
    fn is_gt(p: &Punct) -> bool {
        p.as_char() == '>'
    }

    fn next_is_colon_colon<I>(iter: &std::iter::Peekable<I>) -> bool
    where
        I: Iterator<Item = TokenTree> + Clone,
    {
        let mut look = iter.clone();
        matches!(look.next(), Some(TokenTree::Punct(p)) if p.as_char() == ':')
            && matches!(look.next(), Some(TokenTree::Punct(p)) if p.as_char() == ':')
    }

    fn pass(tokens: TokenStream, field: &str, mut prev_colon_run: u8) -> TokenStream {
        let mut out = Vec::<TokenTree>::new();
        let mut it = tokens.into_iter().peekable();

        while let Some(tt) = it.next() {
            match tt {
                TokenTree::Group(g) => {
                    let inner = pass(g.stream(), field, 0);
                    let mut ng = Group::new(g.delimiter(), inner);
                    ng.set_span(g.span());
                    out.push(TokenTree::Group(ng));
                    prev_colon_run = 0;
                }
                TokenTree::Ident(id) if id == field => {
                    let prev_was_path_sep = prev_colon_run >= 2;
                    let next_starts_with_path_sep = next_is_colon_colon(&it);
                    if prev_was_path_sep || next_starts_with_path_sep {
                        out.push(TokenTree::Ident(id));
                        prev_colon_run = 0;
                        continue;
                    }

                    let mut saw_equals = false;
                    if let Some(TokenTree::Punct(p)) = it.peek()
                        && p.as_char() == '='
                    {
                        saw_equals = true;
                        it.next();
                    }
                    if !saw_equals {
                        out.push(TokenTree::Ident(id));
                        prev_colon_run = 0;
                        continue;
                    }

                    let has_angle =
                        matches!(it.peek(), Some(TokenTree::Punct(p)) if p.as_char() == '<');
                    if !has_angle {
                        out.push(TokenTree::Ident(id));
                        out.push(TokenTree::Punct(Punct::new('=', Spacing::Alone)));
                        prev_colon_run = 0;
                        continue;
                    }

                    it.next(); // consume '<'
                    let mut captured = Vec::<TokenTree>::new();
                    let mut depth: usize = 0;
                    let mut closed = false;

                    for next in it.by_ref() {
                        match next {
                            TokenTree::Punct(p) if is_lt(&p) => {
                                depth += 1;
                                captured.push(TokenTree::Punct(p));
                            }
                            TokenTree::Punct(p) if is_gt(&p) => {
                                if depth == 0 {
                                    closed = true;
                                    break;
                                }
                                depth -= 1;
                                captured.push(TokenTree::Punct(p));
                            }
                            TokenTree::Group(g) => {
                                let inner = pass(g.stream(), field, 0);
                                let mut ng = Group::new(g.delimiter(), inner);
                                ng.set_span(g.span());
                                captured.push(TokenTree::Group(ng));
                            }
                            other => captured.push(other),
                        }
                    }

                    out.push(TokenTree::Ident(id));
                    if closed {
                        let mut paren = TokenStream::new();
                        paren.extend(captured);
                        out.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, paren)));
                    } else {
                        out.push(TokenTree::Punct(Punct::new('=', Spacing::Alone)));
                        out.push(TokenTree::Punct(Punct::new('<', Spacing::Alone)));
                        out.extend(captured);
                    }
                    prev_colon_run = 0;
                }
                TokenTree::Punct(p) => {
                    if p.as_char() == ':' {
                        prev_colon_run = (prev_colon_run.saturating_add(1)).min(2);
                    } else {
                        prev_colon_run = 0;
                    }
                    out.push(TokenTree::Punct(p));
                }
                other => {
                    prev_colon_run = 0;
                    out.push(other);
                }
            }
        }

        out.into_iter().collect()
    }

    pass(tokens, field_name, 0)
}

pub fn maybe_rewrite_generics_angles(tokens: TokenStream, field_name: &str) -> TokenStream {
    #[cfg(feature = "compat_generics_angles")]
    {
        rewrite_generics_angles(tokens, field_name)
    }
    #[cfg(not(feature = "compat_generics_angles"))]
    {
        let _ = field_name;
        tokens
    }
}

#[cfg(all(test, feature = "compat_generics_angles"))]
mod tests {
    #[cfg(feature = "compat_generics_angles")]
    use super::rewrite_generics_angles;
    use internal_test_proc_macro::xtest;
    use quote::quote;

    #[xtest]
    #[cfg(feature = "compat_generics_angles")]
    fn rewrite_generics_angles_basic() {
        let input = quote!(foo(generics = <T, U>));
        let output = rewrite_generics_angles(input, "generics");
        assert_eq!(output.to_string(), quote!(foo(generics(T, U))).to_string());
    }

    #[xtest]
    #[cfg(feature = "compat_generics_angles")]
    fn rewrite_generics_angles_respects_paths() {
        let input = quote!(foo(::generics = <T>, generics = <U>));
        let output = rewrite_generics_angles(input, "generics");
        assert_eq!(output.to_string(), quote!(foo(::generics = <T>, generics(U))).to_string());
    }
}
