#![forbid(unsafe_code)]

//! Procedural macros for Ember.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{spanned::Spanned, DeriveInput, FnArg, ItemImpl, ItemStruct, LitStr, Pat, Path, Token, Type};

fn parse_route_attr(args: TokenStream, macro_name: &str) -> Result<LitStr, TokenStream> {
    match syn::parse2::<LitStr>(args.into()) {
        Ok(lit) => {
            if lit.value().is_empty() {
                let error = syn::Error::new(lit.span(), "route path must not be empty");
                Err(error.to_compile_error().into())
            } else {
                Ok(lit)
            }
        }
        Err(err) => {
            let message = format!("{} expects a single string literal path", macro_name);
            let error = syn::Error::new(err.span(), message);
            Err(error.to_compile_error().into())
        }
    }
}

/// Marks a controller type or impl block.
#[proc_macro_attribute]
pub fn controller(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
    let item_impl = match syn::parse::<ItemImpl>(input_clone) {
        Ok(item) => item,
        Err(_) => return input,
    };

    let self_ty = &item_impl.self_ty;
    let mut routes = Vec::new();
    let mut handlers = Vec::new();

    for item in &item_impl.items {
        if let syn::ImplItem::Fn(method) = item {
            for attr in &method.attrs {
                let ident = attr.path().get_ident().map(|i| i.to_string());
                let Some(ident) = ident else { continue; };

                let method_str = match ident.as_str() {
                    "get" => "GET",
                    "post" => "POST",
                    "put" => "PUT",
                    "patch" => "PATCH",
                    "delete" => "DELETE",
                    "head" => "HEAD",
                    "options" => "OPTIONS",
                    _ => continue,
                };

                let path_lit = match attr.parse_args::<LitStr>() {
                    Ok(lit) => lit,
                    Err(err) => return err.to_compile_error().into(),
                };

                routes.push((method_str, path_lit.clone()));

                let mut args = Vec::new();
                for input in &method.sig.inputs {
                    let FnArg::Typed(pat) = input else { continue; };
                    let Pat::Ident(ident) = &*pat.pat else { continue; };
                    args.push((ident.ident.clone(), (*pat.ty).clone()));
                }

                handlers.push((method_str.to_string(), path_lit, method.sig.ident.clone(), args));
            }
        }
    }

    let route_entries = routes.iter().map(|(method, path)| {
        quote! { ember_core::Route { method: #method, path: #path } }
    });

    let handler_impl = build_http_handler(self_ty, &handlers);
    let expanded = quote! {
        #item_impl

        impl ember_ext_runtime::ControllerMetadata for #self_ty {
            fn routes() -> &'static [ember_core::Route] {
                &[#(#route_entries,)*]
            }
        }

        #handler_impl
    };

    expanded.into()
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(ch.to_ascii_lowercase());
    }
    result
}

fn build_http_handler(
    self_ty: &std::boxed::Box<syn::Type>,
    handlers: &[(String, LitStr, syn::Ident, Vec<(syn::Ident, syn::Type)>)],
) -> proc_macro2::TokenStream {
    let helper_prefix = match self_ty.as_ref() {
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|seg| {
                let name = seg.ident.to_string();
                let snake = to_snake_case(&name);
                format_ident!("{}", snake)
            })
            .unwrap_or_else(|| format_ident!("controller")),
        _ => format_ident!("controller"),
    };

    let decode_fn = format_ident!("__ember_url_decode_{}", helper_prefix);
    let from_hex_fn = format_ident!("__ember_from_hex_{}", helper_prefix);
    let query_fn = format_ident!("__ember_query_param_{}", helper_prefix);

    let mut arms = Vec::new();
    for (method_str, path_lit, fn_ident, args) in handlers {
        let method_lit = LitStr::new(method_str, path_lit.span());
        let path_value = path_lit.value();

        let (prefix, param_name, suffix) = parse_path_template(&path_value);

        let status = if method_str == "POST" { 201u16 } else { 200u16 };

        let mut arg_builders = Vec::new();
        let mut body_arg: Option<(syn::Ident, syn::Type)> = None;

        for (arg_ident, arg_ty) in args {
            let arg_name = arg_ident.to_string();
            if let Some(param) = &param_name {
                if param == &arg_name {
                    let parse_expr = build_parse_expr(arg_ident, arg_ty, quote! { param_value });
                    arg_builders.push(parse_expr);
                    continue;
                }
            }

            if method_str == "GET" {
                let parse_expr = build_query_expr(arg_ident, arg_ty, &query_fn);
                arg_builders.push(parse_expr);
                continue;
            }

            if matches!(method_str.as_str(), "POST" | "PUT" | "PATCH") {
                if body_arg.is_some() {
                    let err = syn::Error::new(path_lit.span(), "only one body parameter supported");
                    return err.to_compile_error();
                }
                body_arg = Some((arg_ident.clone(), arg_ty.clone()));
            }
        }

        if let Some((arg_ident, arg_ty)) = body_arg {
            let parse_expr = quote! {
                let #arg_ident: #arg_ty = ::serde_json::from_slice(body)
                    .map_err(|err| ember_core::EmberError::msg(format!("invalid JSON: {err}")))?;
            };
            arg_builders.push(parse_expr);
        }

        let call_args = args.iter().map(|(ident, _)| ident);
        let call = quote! {
            let ember_core::Json(payload) = self.#fn_ident(#(#call_args),*);
            let body = ::serde_json::to_vec(&payload)
                .map_err(|err| ember_core::EmberError::msg(format!("encode failed: {err}")))?;
            Ok(ember_core::HttpResponse {
                status: #status,
                content_type: "application/json",
                body,
            })
        };

        let arm = if param_name.is_some() {
            let prefix_lit = LitStr::new(&prefix, path_lit.span());
            let suffix_lit = LitStr::new(&suffix, path_lit.span());
            let guard = if suffix.is_empty() {
                quote! { path.starts_with(#prefix_lit) }
            } else {
                quote! { path.starts_with(#prefix_lit) && path.ends_with(#suffix_lit) }
            };
            quote! {
                (#method_lit, path) if #guard => {
                    let start = #prefix_lit.len();
                    let end = path.len().saturating_sub(#suffix_lit.len());
                    let param_value = &path[start..end];
                    #(#arg_builders)*
                    #call
                }
            }
        } else {
            quote! {
                (#method_lit, #path_lit) => {
                    #(#arg_builders)*
                    #call
                }
            }
        };

        arms.push(arm);
    }

    quote! {
        impl ember_core::HttpHandler for #self_ty {
            fn handle(&self, method: &str, path: &str, body: &[u8]) -> Result<ember_core::HttpResponse, ember_core::EmberError> {
                let (path_only, query) = match path.split_once('?') {
                    Some((p, q)) => (p, Some(q)),
                    None => (path, None),
                };
                match (method, path_only) {
                    #(#arms,)*
                    _ => Ok(ember_core::HttpResponse::text(404, "not found")),
                }
            }
        }

        fn #query_fn(query: Option<&str>, name: &str) -> Option<String> {
            let query = query?;
            let target = format!("{}=", name);
            query
                .split('&')
                .find(|pair| pair.starts_with(&target))
                .and_then(|pair| pair.split_once('=').map(|(_, value)| #decode_fn(value)))
        }

        fn #decode_fn(value: &str) -> String {
            let mut out = String::with_capacity(value.len());
            let mut chars = value.as_bytes().iter().copied();
            while let Some(ch) = chars.next() {
                match ch {
                    b'+' => out.push(' '),
                    b'%' => {
                        let hi = chars.next();
                        let lo = chars.next();
                        if let (Some(hi), Some(lo)) = (hi, lo) {
                            if let (Some(hi), Some(lo)) = (#from_hex_fn(hi), #from_hex_fn(lo)) {
                                out.push((hi << 4 | lo) as char);
                            }
                        }
                    }
                    _ => out.push(ch as char),
                }
            }
            out
        }

        fn #from_hex_fn(value: u8) -> Option<u8> {
            match value {
                b'0'..=b'9' => Some(value - b'0'),
                b'a'..=b'f' => Some(value - b'a' + 10),
                b'A'..=b'F' => Some(value - b'A' + 10),
                _ => None,
            }
        }
    }
}

fn parse_path_template(path: &str) -> (String, Option<String>, String) {
    if let Some(start) = path.find('{') {
        if let Some(end) = path[start + 1..].find('}') {
            let end = start + 1 + end;
            let prefix = path[..start].to_string();
            let param = path[start + 1..end].to_string();
            let suffix = path[end + 1..].to_string();
            return (prefix, Some(param), suffix);
        }
    }
    (path.to_string(), None, String::new())
}

fn is_string_type(ty: &Type) -> bool {
    if let Type::Path(path) = ty {
        if path.qself.is_none() && path.path.segments.len() == 1 {
            return path.path.segments[0].ident == "String";
        }
    }
    false
}

fn build_parse_expr(ident: &syn::Ident, ty: &Type, source: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    if is_string_type(ty) {
        quote! { let #ident: #ty = #source.to_string(); }
    } else {
        quote! {
            let #ident: #ty = #source
                .parse::<#ty>()
                .map_err(|_| ember_core::EmberError::msg("invalid parameter"))?;
        }
    }
}

fn build_query_expr(
    ident: &syn::Ident,
    ty: &Type,
    query_fn: &syn::Ident,
) -> proc_macro2::TokenStream {
    let name = ident.to_string();
    if is_string_type(ty) {
        quote! {
            let #ident: #ty = #query_fn(query, #name)
                .unwrap_or_default();
        }
    } else {
        quote! {
            let raw = #query_fn(query, #name)
                .ok_or_else(|| ember_ext_exceptions::EmberError::msg("missing query parameter"))?;
            let #ident: #ty = raw
                .parse::<#ty>()
                .map_err(|_| ember_ext_exceptions::EmberError::msg("invalid query parameter"))?;
        }
    }
}

/// Marks a GET handler and validates the route attribute shape.
#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    match parse_route_attr(args, "#[get]") {
        Ok(_lit) => input,
        Err(err) => err,
    }
}

/// Marks a POST handler and validates the route attribute shape.
#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    match parse_route_attr(args, "#[post]") {
        Ok(_lit) => input,
        Err(err) => err,
    }
}

/// Marks a PUT handler and validates the route attribute shape.
#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    match parse_route_attr(args, "#[put]") {
        Ok(_lit) => input,
        Err(err) => err,
    }
}

/// Marks a PATCH handler and validates the route attribute shape.
#[proc_macro_attribute]
pub fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    match parse_route_attr(args, "#[patch]") {
        Ok(_lit) => input,
        Err(err) => err,
    }
}

/// Marks a DELETE handler and validates the route attribute shape.
#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    match parse_route_attr(args, "#[delete]") {
        Ok(_lit) => input,
        Err(err) => err,
    }
}

/// Marks a HEAD handler and validates the route attribute shape.
#[proc_macro_attribute]
pub fn head(args: TokenStream, input: TokenStream) -> TokenStream {
    match parse_route_attr(args, "#[head]") {
        Ok(_lit) => input,
        Err(err) => err,
    }
}

/// Marks an OPTIONS handler and validates the route attribute shape.
#[proc_macro_attribute]
pub fn options(args: TokenStream, input: TokenStream) -> TokenStream {
    match parse_route_attr(args, "#[options]") {
        Ok(_lit) => input,
        Err(err) => err,
    }
}

/// Marks a service type for DI registration.
#[proc_macro_attribute]
pub fn service(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// Marks a configuration type for binding.
#[proc_macro_attribute]
pub fn config(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// Marks an entity struct and implements the Ember Entity trait.
///
/// Usage: #[entity(id = "id_field", table = "table_name")]
#[proc_macro_attribute]
pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = match syn::parse2::<EntityArgs>(args.into()) {
        Ok(args) => args,
        Err(err) => return err.to_compile_error().into(),
    };

    let id_field = args.id;
    let table_name = args.table;

    let id_field = match id_field {
        Some(field) => field,
        None => {
            let err = syn::Error::new(proc_macro2::Span::call_site(), "#[entity] requires id = \"field\"");
            return err.to_compile_error().into();
        }
    };

    let input_ast = match syn::parse::<ItemStruct>(input.clone()) {
        Ok(item) => item,
        Err(err) => return err.to_compile_error().into(),
    };

    let ident = &input_ast.ident;
    let fields = match &input_ast.fields {
        syn::Fields::Named(fields) => &fields.named,
        _ => {
            let err = syn::Error::new(input_ast.span(), "#[entity] only supports structs with named fields");
            return err.to_compile_error().into();
        }
    };

    let mut id_ty = None;
    for field in fields {
        if let Some(field_ident) = &field.ident {
            if field_ident == &id_field {
                id_ty = Some(field.ty.clone());
                break;
            }
        }
    }

    let id_ty = match id_ty {
        Some(ty) => ty,
        None => {
            let err = syn::Error::new(input_ast.span(), format!("id field '{}' not found", id_field));
            return err.to_compile_error().into();
        }
    };

    let id_ident = syn::Ident::new(&id_field, proc_macro2::Span::call_site());
    let table_name = table_name.unwrap_or_else(|| ident.to_string().to_lowercase());

    let mut columns = Vec::new();
    for field in fields {
        let Some(_field_ident) = &field.ident else { continue; };
        let field_name = match column_name(field) {
            Ok(name) => name,
            Err(err) => return err.to_compile_error().into(),
        };
        let (sql_type, not_null) = map_sql_type(&field.ty);
        let mut column = format!("{} {}", field_name, sql_type);
        if field_name == id_field {
            column.push_str(" PRIMARY KEY");
        } else if not_null {
            column.push_str(" NOT NULL");
        }
        columns.push(column);
    }
    let create_sql = format!(
        "CREATE TABLE IF NOT EXISTS {} ({})",
        table_name,
        columns.join(", ")
    );
    let create_sql_lit = LitStr::new(&create_sql, proc_macro2::Span::call_site());
    let expanded = quote! {
        #input_ast

        impl ember_ext_db::Entity for #ident {
            type Id = #id_ty;

            fn id(&self) -> Self::Id {
                self.#id_ident.clone()
            }
        }

        ember_ext_db::inventory::submit! {
            ember_ext_db::EntityMigration { sql: #create_sql_lit }
        }
    };

    expanded.into()
}

fn map_sql_type(ty: &syn::Type) -> (&'static str, bool) {
    if let syn::Type::Path(type_path) = ty {
        if type_path.path.segments.len() == 1 {
            let ident = type_path.path.segments[0].ident.to_string();
            return match ident.as_str() {
                "String" => ("TEXT", true),
                "bool" => ("BOOLEAN", true),
                "i64" | "u64" | "isize" | "usize" => ("BIGINT", true),
                "i32" | "u32" | "i16" | "u16" | "i8" | "u8" => ("INTEGER", true),
                _ => ("TEXT", true),
            };
        }

        if let Some(seg) = type_path.path.segments.first() {
            if seg.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                        let (sql_type, _) = map_sql_type(inner);
                        return (sql_type, false);
                    }
                }
            }
        }
    }

    ("TEXT", true)
}

struct EntityArgs {
    id: Option<String>,
    table: Option<String>,
}

impl syn::parse::Parse for EntityArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut id: Option<String> = None;
        let mut table: Option<String> = None;
        while !input.is_empty() {
            let key: Path = input.parse()?;
            let _eq: Token![=] = input.parse()?;
            let value: LitStr = input.parse()?;

            if key.is_ident("id") {
                id = Some(value.value());
            } else if key.is_ident("table") {
                table = Some(value.value());
            } else {
                return Err(syn::Error::new(key.span(), "unsupported argument"));
            }

            if input.peek(Token![,]) {
                let _comma: Token![,] = input.parse()?;
            }
        }

        Ok(Self { id, table })
    }
}

fn column_name(field: &syn::Field) -> syn::Result<String> {
    for attr in &field.attrs {
        if attr.path().is_ident("column") {
            let args = attr.parse_args::<ColumnArgs>()?;
            return Ok(args.name);
        }
    }

    field
        .ident
        .as_ref()
        .map(|ident| ident.to_string())
        .ok_or_else(|| syn::Error::new(field.span(), "unnamed field"))
}

struct ColumnArgs {
    name: String,
}

impl syn::parse::Parse for ColumnArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key: Path = input.parse()?;
        let _eq: Token![=] = input.parse()?;
        let value: LitStr = input.parse()?;

        if !key.is_ident("name") {
            return Err(syn::Error::new(key.span(), "expected name = \"column\""));
        }

        Ok(Self { name: value.value() })
    }
}

/// Marks a repository type (placeholder for future codegen).
#[proc_macro_attribute]
pub fn repository(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// Optional derive macro for configuration types.
#[proc_macro_derive(EmberConfig)]
pub fn derive_ember_config(input: TokenStream) -> TokenStream {
    let input_ast = syn::parse_macro_input!(input as DeriveInput);
    let ident = input_ast.ident.clone();
    let expanded = quote! {
        impl #ident {
            /// Placeholder derived hook for Ember config types.
            pub fn ember_config_marker(&self) -> &'static str {
                "ember-config"
            }
        }
    };
    expanded.into()
}
