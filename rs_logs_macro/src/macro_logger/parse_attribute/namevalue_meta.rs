use super::types::ParsedAttribute;

// struct NameValueAttr<'a> {
//     args: Option<&'a str>,
// }
// impl<'a> syn::parse::Parse for NameValueAttr<'a> {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         Ok(Self { args: None })
//     }
// }

pub fn parse_namevalue_meta<'a>(
    meta: &'a syn::MetaNameValue,
    attr: &'a syn::Attribute,
) -> syn::Result<Option<ParsedAttribute<'a>>> {
    let ident = &meta.path.segments[0].ident;

    match ident.to_string().as_str() {
        "crate_idx" => {
            let val = parse_int_attr(&meta.value);
            // eprintln!("{:?}", val);
        }
        _ => (),
    }

    // let x = meta.value;

    // match &meta.value {
    //     syn::Expr::Lit(lit_expr) => match &lit_expr.lit {
    //         syn::Lit::Str(val) => {
    //             let y = val.value();
    //             // let x = val.token();
    //             eprintln!("X: {:?}", y);
    //         }
    //         // sy
    //         _ => (),
    //     },
    //     _ => (),
    // }

    // &meta.parse()

    // let x = meta
    // let vals = meta.value

    let parsed_attr = ParsedAttribute {
        ident,
        args: vec![],
    };

    // eprintln!("{:?}", meta.value);

    // let data = syn::Attribute::parse_inner(input)

    Ok(Some(parsed_attr))
}

fn parse_int_attr<'a>(meta_value: &'a syn::Expr) -> Option<usize> {
    match meta_value {
        syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Int(lit_int) => match lit_int.base10_parse::<usize>() {
                Ok(value) => Some(value),
                Err(_) => None,
            },
            _ => None,
        },
        _ => None,
    }
}

fn parse_str_attr<'a>(meta_value: &'a syn::Expr) -> Option<String> {
    match meta_value {
        syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Str(lit_str) => {
                let value = lit_str.value();
                Some(value)
            }
            _ => None,
        },
        _ => None,
    }
}

// fn parse_str_attr<'a>(meta_value: &'a syn::Expr) -> syn::Result<>
