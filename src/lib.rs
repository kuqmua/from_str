#[proc_macro_derive(FromStr)]
pub fn from_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location(); //panic_location function from https://github.com/kuqmua/proc_macro_helpers
    let proc_macro_name = "from_str";
    let ast: syn::DeriveInput = syn::parse(input)
        .unwrap_or_else(|_| panic!("let ast: syn::DeriveInput = syn::parse(input) failed"));
    let ident = &ast.ident;
    let proc_macro_name_ident_stringified = format!("{proc_macro_name} {ident}");
    let data_enum = if let syn::Data::Enum(data_enum) = ast.data {
        data_enum
    } else {
        panic!(
            "{proc_macro_name_ident_stringified} {} syn::Data::Enum",
            proc_macro_helpers::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
        );
    };

    let gen = quote::quote! {
        // impl std::str::FromStr for #ident {
        //     type Err = std::string::String;
        //     fn from_str(value: &str) -> Result<Self, Self::Err> {
        //         match value {
        //             "id" => Ok(Self::Id),
        //             "name" => Ok(Self::Name),
        //             "color" => Ok(Self::Color),
        //             _ => Err(format!("Invalid CatOrderByColumn, expected one of \'id\', \'name\', \'color\', found {value}")),
        //         }
        //     }
        // }
    };
    if ident == "" {
        //println!("{gen}");
    }
    gen.into()
}
