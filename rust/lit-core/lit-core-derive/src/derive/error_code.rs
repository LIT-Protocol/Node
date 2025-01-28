use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::ResultExt;
use proc_macro_error::{abort, abort_call_site};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{self, parenthesized, Attribute, Data, DeriveInput, Expr, Ident, LitStr, Token};

pub(crate) fn derive_error_code(input: &DeriveInput) -> TokenStream {
    let name = input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match input.data {
        Data::Enum(ref e) => {
            let mut code_variants: Vec<TokenStream2> = Vec::new();
            let mut kind_variants: Vec<TokenStream2> = Vec::new();
            let mut http_code_variants: Vec<TokenStream2> = Vec::new();

            for variant in e.variants.iter() {
                let variant_name = variant.ident.clone();
                let variant_name_str = variant_name.to_string();

                let mut added_kind = false;
                let mut added_http_code = false;

                code_variants.push(quote! {
                    #name::#variant_name => Cow::from(#variant_name_str),
                });

                for attr in CodeAttr::parse_all(&variant.attrs).iter() {
                    if let Some(magic) = attr.magic.as_ref() {
                        if let Some(val) = attr.value.as_ref() {
                            match magic {
                                MagicAttrName::Kind => {
                                    kind_variants.push(quote! {
                                        #name::#variant_name => Some(::lit_core::error::Kind::#val),
                                    });
                                    added_kind = true;
                                }
                                MagicAttrName::HttpStatus => {
                                    http_code_variants.push(quote! {
                                        #name::#variant_name => Some(#val),
                                    });
                                    added_http_code = true;
                                }
                            }
                        }
                    }
                }

                if !added_kind {
                    kind_variants.push(quote! {
                        #name::#variant_name => None,
                    });
                }
                if !added_http_code {
                    http_code_variants.push(quote! {
                        #name::#variant_name => None,
                    });
                }
            }

            let modified = quote! {
                impl #impl_generics ::lit_core::error::Code for #name #ty_generics #where_clause {
                    fn code(&self) -> Cow<str> {
                        match self {
                            #(#code_variants)*
                        }
                    }

                    fn kind(&self) -> Option<Kind> {
                        match self {
                            #(#kind_variants)*
                        }
                    }

                    fn http_status(&self) -> Option<u16> {
                        match self {
                            #(#http_code_variants)*
                        }
                    }
                }
            };

            TokenStream::from(modified)
        }
        _ => abort_call_site!("`#[derive(ErrorCode)]` only supports enums"),
    }
}

#[derive(Clone)]
pub struct CodeAttr {
    valid: bool,
    pub magic: Option<MagicAttrName>,
    pub value: Option<AttrValue>,
}

// Copied code, could be vastly simplified.
impl CodeAttr {
    pub fn parse_all(all_attrs: &[Attribute]) -> Vec<Self> {
        all_attrs
            .iter()
            .filter_map(|attr| {
                let valid = if attr.path.is_ident("code") { Some(true) } else { None };
                valid.map(|v| (v, attr))
            })
            .flat_map(|(v, attr)| {
                attr.parse_args_with(Punctuated::<CodeAttr, Token![,]>::parse_terminated)
                    .unwrap_or_abort()
                    .into_iter()
                    .map(move |mut a| {
                        a.valid = v;
                        a
                    })
            })
            .filter(|a| a.valid)
            .collect()
    }
}

impl Parse for CodeAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        let magic = match name_str.as_str() {
            "kind" => Some(MagicAttrName::Kind),
            "http_status" => Some(MagicAttrName::HttpStatus),
            _ => None,
        };

        let value = if input.peek(Token![=]) {
            // `name = value` attributes.
            let assign_token = input.parse::<Token![=]>()?; // skip '='
            if input.peek(LitStr) {
                let lit: LitStr = input.parse()?;
                Some(AttrValue::LitStr(lit))
            } else {
                match input.parse::<Expr>() {
                    Ok(expr) => Some(AttrValue::Expr(expr)),

                    Err(_) => abort! {
                        assign_token,
                        "expected `string literal` or `expression` after `=`"
                    },
                }
            }
        } else if input.peek(syn::token::Paren) {
            // `name(...)` attributes.
            let nested;
            parenthesized!(nested in input);

            let method_args: Punctuated<_, Token![,]> = nested.parse_terminated(Expr::parse)?;
            Some(AttrValue::Call(Vec::from_iter(method_args)))
        } else {
            None
        };

        Ok(Self {
            valid: false, // Updated later.
            magic,
            value,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MagicAttrName {
    Kind,
    HttpStatus,
}

#[derive(Clone)]
pub enum AttrValue {
    LitStr(LitStr),
    Expr(Expr),
    Call(Vec<Expr>),
}

impl ToTokens for AttrValue {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::LitStr(t) => t.to_tokens(tokens),
            Self::Expr(t) => t.to_tokens(tokens),
            Self::Call(t) => {
                let t = quote!(#(#t),*);
                t.to_tokens(tokens)
            }
        }
    }
}
