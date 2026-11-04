use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Expr, Ident, ItemFn, LitInt, LitStr, Result, ReturnType, Stmt, Token, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

struct EndpointArgs {
    method: Ident,
    url: LitStr,
}

impl Parse for EndpointArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let method = input.parse()?;
        input.parse::<Token![,]>()?;
        let url = input.parse()?;
        Ok(Self { method, url })
    }
}

struct Pair {
    key: LitStr,
    value: Expr,
}

impl Parse for Pair {
    fn parse(input: ParseStream) -> Result<Self> {
        let key = input.parse()?;
        input.parse::<Token![=>]>()?;
        let value = input.parse()?;
        Ok(Self { key, value })
    }
}

struct PagingArgs {
    var: Ident,
    limit: LitInt,
}

impl Parse for PagingArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let var = input.parse()?;
        input.parse::<Token![,]>()?;
        let key: Ident = input.parse()?;
        if key != "limit" {
            return Err(syn::Error::new(key.span(), "expected `limit`"));
        }
        input.parse::<Token![=]>()?;
        let limit = input.parse()?;
        Ok(Self { var, limit })
    }
}

struct MapArgs {
    pat: Ident,
    ty: Type,
    expr: Expr,
}

impl Parse for MapArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![|]>()?;
        let pat = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty = input.parse()?;
        input.parse::<Token![|]>()?;
        let expr = input.parse()?;
        Ok(Self { pat, ty, expr })
    }
}

#[proc_macro_attribute]
pub fn endpoint(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as EndpointArgs);
    let func = parse_macro_input!(item as ItemFn);

    match expand(args, func) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand(args: EndpointArgs, func: ItemFn) -> Result<TokenStream2> {
    let method = method_tokens(&args.method)?;
    let url = &args.url;

    let mut prelude = Vec::<Stmt>::new();
    let mut query = Vec::<Pair>::new();
    let mut headers = Vec::<Pair>::new();
    let mut paging = None::<PagingArgs>;
    let mut body = None::<Expr>;
    let mut map = None::<MapArgs>;
    let mut try_map = None::<MapArgs>;
    let mut raw_bytes = false;
    let mut void = false;

    for statement in func.block.stmts {
        if let Stmt::Macro(item) = &statement {
            let name = item.mac.path.get_ident().map(Ident::to_string);

            match name.as_deref() {
                Some("query") => {
                    query.extend(
                        item.mac
                            .parse_body_with(Punctuated::<Pair, Token![,]>::parse_terminated)?,
                    );
                    continue;
                }
                Some("headers") => {
                    headers.extend(
                        item.mac
                            .parse_body_with(Punctuated::<Pair, Token![,]>::parse_terminated)?,
                    );
                    continue;
                }
                Some("paging") => {
                    paging = Some(item.mac.parse_body()?);
                    continue;
                }
                Some("body") => {
                    body = Some(item.mac.parse_body()?);
                    continue;
                }
                Some("map") => {
                    map = Some(item.mac.parse_body()?);
                    continue;
                }
                Some("try_map") => {
                    try_map = Some(item.mac.parse_body()?);
                    continue;
                }
                Some("raw_bytes") => {
                    raw_bytes = true;
                    continue;
                }
                Some("void") => {
                    void = true;
                    continue;
                }
                _ => {}
            }
        }

        prelude.push(statement);
    }

    let paging_code = match &paging {
        Some(paging) => {
            let var = &paging.var;
            let limit = &paging.limit;
            quote! {
                let __pq_limit = #var.limit.unwrap_or(#limit).to_string();
                let __pq_order = #var.order.unwrap_or_default().to_string();
                let __pq_cursor = match #var.cursor {
                    Some(cursor) => cursor.to_string(),
                    None => ::std::string::String::new(),
                };
                __query_pairs.push(("limit", &__pq_limit));
                __query_pairs.push(("sortOrder", &__pq_order));
                __query_pairs.push(("cursor", &__pq_cursor));
            }
        }
        None => TokenStream2::new(),
    };

    let query_push = query.iter().map(|pair| {
        let key = &pair.key;
        let value = &pair.value;
        quote! { __query_pairs.push((#key, #value)); }
    });

    let header_push = headers.iter().map(|pair| {
        let key = &pair.key;
        let value = &pair.value;
        quote! {
            __headers.insert(
                ::roblox_api::reqwest::header::HeaderName::from_bytes(#key.as_bytes())
                    .map_err(|_| ::roblox_api::Error::InvalidHeader)?,
                ::roblox_api::reqwest::header::HeaderValue::from_str(&#value)
                    .map_err(|_| ::roblox_api::Error::InvalidHeader)?,
            );
        }
    });

    let request_body = match &body {
        Some(body) => quote! { Some(&#body) },
        None => quote! { None::<&()> },
    };

    let ret = match &func.sig.output {
        ReturnType::Type(_, ty) => quote! { #ty },
        ReturnType::Default => quote! { () },
    };

    let decode = if let Some(map) = &map {
        let (pat, ty, expr) = (&map.pat, &map.ty, &map.expr);
        quote! { __response.json::<#ty>().await.map(|#pat| #expr) }
    } else if let Some(map) = &try_map {
        let (pat, ty, expr) = (&map.pat, &map.ty, &map.expr);
        quote! { __response.json::<#ty>().await.and_then(|#pat| #expr) }
    } else if raw_bytes {
        quote! { __response.bytes().await }
    } else if void {
        quote! { Ok(()) }
    } else {
        quote! { __response.json::<#ret>().await }
    };

    let attrs = &func.attrs;
    let name = &func.sig.ident;
    let params = &func.sig.inputs;

    Ok(quote! {
        #(#attrs)*
        #[allow(unused_variables)]
        pub async fn #name(
            client: &mut ::roblox_api::client::Client,
            #params
        ) -> ::std::result::Result<#ret, ::roblox_api::Error> {
            #(#prelude)*

            let mut __query_pairs: ::std::vec::Vec<(&str, &str)> = ::std::vec::Vec::new();
            #paging_code

            let url: ::std::string::String = ::std::format!(#url);
            #(#query_push)*
            let query = if __query_pairs.is_empty() {
                None
            } else {
                Some(__query_pairs.as_slice())
            };

            #[allow(unused_mut)]
            let mut __headers = ::roblox_api::reqwest::header::HeaderMap::new();
            #(#header_push)*
            let __headers = if __headers.is_empty() {
                None
            } else {
                Some(__headers)
            };

            let __response = client
                .requestor
                .request(#method, &url, #request_body, query, __headers)
                .await?;

            #decode
        }
    })
}

fn method_tokens(method: &Ident) -> Result<TokenStream2> {
    let tokens = match method.to_string().as_str() {
        "GET" => quote! { ::roblox_api::reqwest::Method::GET },
        "POST" => quote! { ::roblox_api::reqwest::Method::POST },
        "PUT" => quote! { ::roblox_api::reqwest::Method::PUT },
        "PATCH" => quote! { ::roblox_api::reqwest::Method::PATCH },
        "DELETE" => quote! { ::roblox_api::reqwest::Method::DELETE },
        other => {
            return Err(syn::Error::new(
                method.span(),
                format!("unsupported HTTP method `{other}`"),
            ));
        }
    };

    Ok(tokens)
}
