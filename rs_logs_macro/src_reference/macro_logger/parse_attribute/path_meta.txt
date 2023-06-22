use super::types::ParsedAttribute;

// Example: #[ident]
pub fn parse_path_meta<'a>(
  meta: &'a syn::Path,
  attr: &'a syn::Attribute,
) -> syn::Result<Option<ParsedAttribute<'a>>> {
  let ident = &meta.segments[0].ident;
  eprintln!("PATH: {:?}", ident);

  match ident.to_string().as_str() {
      "config" => {
          return Err(syn::Error::new_spanned(
              attr,
              "The 'config' attribute is missing configuration arguments to be passed\n\nExample:\n#[config(\n    crate_idx = 1,\n    mod_idx = 2,\n    location: \"crate::mod1::example\"\n)]\n",
          ))
      }
      "info_msg" => {
          return Err(syn::Error::new_spanned(
              attr,
              "The 'info_msg' attribute requires a string argument to be passed\n\nExample:\n#[info_msg(\"my log message\")]\n",
          ));
      }
      "warn_msg" => {
          return Err(syn::Error::new_spanned(
              attr,
              "The 'warn_msg' attribute requires a string argument to be passed\n\nExample:\n#[warn_msg(\"my log message\")]\n",
          ));
      }
      "error_msg" => {
        return Err(syn::Error::new_spanned(
          attr, 
          "The 'error_msg' attribute requires a string argument to be passed\n\nExample:\n#[error_msg(\"my log message\")]\n",
        ));
      }
      _ => (),
  }

  let parsed_attr = ParsedAttribute {
      ident,
      args: vec![],
  };

  Ok(Some(parsed_attr))
}
