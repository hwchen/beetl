use syn::parse::Nothing;
use syn::{Field, Result};

pub fn is_value_var(field: &Field) -> Result<bool> {
    for attr in &field.attrs {
        if attr.path.is_ident("value_var") {
            syn::parse2::<Nothing>(attr.tokens.clone())?;
            return Ok(true);
        }
    }
    Ok(false)
}
