use super::types::ParsedAttribute;

struct ArgumentParser<'a> {
    args: Vec<&'a str>,
}
impl<'a> syn::parse::Parse for ArgumentParser<'a> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args: Vec<&'a str> = vec![];

        Ok(Self { args })
    }
}
// struct Arg<'a> {
//     //
// }

// Example: #[ident(args)]
pub fn parse_list_meta<'a>(
    meta: &'a syn::MetaList,
    attrs: &'a syn::Attribute,
) -> syn::Result<Option<ParsedAttribute<'a>>> {
    let ident = &meta.path.segments[0].ident;
    // eprintln!("LIST: {:?}", ident.to_string());

    // let data: ArgumentParser = meta.parse_args()?;
    // let d2 = meta.parse_args_with(|input| {
    //     //
    //     Ok(())
    // });

    // meta.parse_nested_meta(logic)

    // let x = syn::parse2::<syn::MetaList>(meta.tokens.clone())?;
    // let y = x.parse_args();
    // x.tokens.

    // eprintln!("{:#?}", attrs);

    // let args: syn::Expr = attrs.parse_args()?;
    // let args: syn::parse::ParseStream = attrs.parse_args()?;

    // eprintln!("{:#?}\n", meta.tokens);

    // let x = &meta.tokens;
    // eprintln!("{:#?}\n{}", x, x.into_iter());

    // let x = attrs.parse_args_with(syn::Attribute::parse_outer)?;

    // let x = match ident.to_string().as_str() {
    //   "info_msg" => {
    //     // eprintln!("{:#?}", meta.tokens);
    //     // let args
    //     Some(23)
    //   },
    //     _ => None
    // };

    Ok(None)
}
