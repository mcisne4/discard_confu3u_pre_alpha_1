pub fn get_enum_def_tokens(enum_ast: &venial::Enum) -> proc_macro2::TokenStream {
    let mut tokens: Vec<proc_macro2::TokenTree> = vec![];

    if let Some(visibility) = &enum_ast.vis_marker {
        tokens.push(visibility.tk_token1.clone());
    }

    let enum_kw: proc_macro2::TokenTree = enum_ast.tk_enum.clone().into();
    tokens.push(enum_kw);

    let ident: proc_macro2::TokenTree = enum_ast.name.clone().into();
    tokens.push(ident);

    proc_macro2::TokenStream::from_iter(tokens.into_iter())
}
