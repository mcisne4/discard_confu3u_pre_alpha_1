use super::super::attr_errors::AttrError;
use super::ParsedAttr;

struct StringAttrData(String);
impl syn::parse::Parse for StringAttrData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            eprintln!("\tinput is empty");
            eprintln!("{:?}", input);
            input.error("No Data");
        } else {
            eprintln!("\tinput is not empty");
            eprintln!("{:?}", input);
        }

        let mut data = String::from("Apple: ");

        // loop {
        //     if input.is_empty() {
        //         break;
        //     }

        //     break;
        // }

        Ok(Self(data))
    }
}

pub fn parse_sting_attr<'a>(
    ident: &'a syn::Ident,
    meta: &'a syn::MetaList,
    attr: &'a syn::Attribute,
) -> syn::Result<ParsedAttr<'a>> {
    let token_stream = &meta.tokens;
    eprintln!("\n{}", ident.to_string());

    // let mut token_stream = token_stream.clone();

    // for token in token_stream.into_iter() {
    //     //
    // }

    // let x = &meta.parse_args_with(|f|)

    match meta.parse_args::<StringAttrData>() {
        Ok(data) => eprintln!("\tSUCCESS: {:?}", data.0),
        Err(err) => eprintln!("\tERROR: {:?}", err.to_string()),
    }

    // for token_tree in token_stream.clone().into_iter() {
    //     eprintln!("{:?}", token_tree);
    // }

    // let x = token_stream.into_iter().map(|token| {
    //   eprintln!("{:?}", token);
    // })

    // todo!()
    Ok(ParsedAttr {
        ident,
        value: String::from("To Be Implemented"),
        attr,
    })
}
