use syn::Lit;

fn lit_to_unquoted_string(lit: &Lit) -> String {
    match lit {
        Lit::Str(s) => s.value(), // removes surrounding quotes
        Lit::ByteStr(bs) => String::from_utf8_lossy(&bs.value()).to_string(),
        Lit::Byte(b) => b.value().to_string(),
        Lit::Char(c) => c.value().to_string(),
        Lit::Int(i) => i.base10_digits().to_string(),
        Lit::Float(f) => f.base10_digits().to_string(),
        Lit::Bool(b) => b.value.to_string(),
        Lit::Verbatim(v) => v.to_string(),
        _ => unimplemented!(),
    }
}

pub trait LitExt {
    fn unquoted_string(&self) -> String;
}

impl LitExt for Lit {
    fn unquoted_string(&self) -> String {
        lit_to_unquoted_string(self)
    }
}
