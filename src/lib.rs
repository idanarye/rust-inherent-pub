extern crate proc_macro;

use proc_macro::{TokenStream};
use syn::{
    parse_macro_input,
    ItemImpl,
    ImplItem,
    Visibility,
    Signature,
    Path,
    FnArg,
    Pat,
    PatIdent,
    Ident,
};
use syn::spanned::Spanned;
use syn::parse::Error;
use quote::{quote, ToTokens};

/// Make `pub` methods in the annotated `impl Trait for Type` block into
/// inherited methods, which can be used without having to import the trait.
///
/// ```
/// mod module {
///     # use inherent_pub::inherent_pub;
///     pub trait Foo {
///         fn foo(self);
///         fn bar(self);
///     }
///
///     pub struct Bar;
///
///     #[inherent_pub]
///     impl Foo for Bar {
///         // `foo` becomes an inherent method.
///         pub fn foo(self) {}
///         // `bar` is not an inherent method (not `pub`)
///         fn bar(self) {}
///     }
/// }
///
/// fn main() {
///     // We didn't `use foo:Foo`, but we can still use `Bar.foo()`:
///     module::Bar.foo();
///
///     // This does not compile:
///     // bar::Bar.bar();
///
///     {
///         // We need to import the trait in order to use `Bar.bar()`:
///         use module::Foo;
///         module::Bar.bar();
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn inherent_pub(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input: ItemImpl = parse_macro_input!(input as ItemImpl);

    match inherent_pub_impl(input) {
        Ok(output) => {
            output
        },
        Err(error) => {
            error.to_compile_error().into()
        }
    }
}

fn inherent_pub_impl(mut input: ItemImpl) -> Result<TokenStream, Error> {
    let (_, trait_, _) = input.trait_.clone().ok_or_else(|| {
        Error::new(input.span(), "expected `impl <Trait> for <Type>`")
    }).unwrap();

    let methods = extract_pub_methods(&mut input.items);

    let mut result = TokenStream::new().into();
    input.to_tokens(&mut result);

    let pub_impls = methods.into_iter().map(|(vis, sig)| {
        redirect_method(vis, sig, &trait_)
    }).collect::<Result<Vec<_>, Error>>()?;

    let inherent_impl = ItemImpl {
        trait_: None,
        items: pub_impls,
        ..input
    };
    inherent_impl.to_tokens(&mut result);

    Ok(result.into())
}

fn extract_pub_methods(items: &mut Vec<ImplItem>) -> Vec<(Visibility, Signature)> {
    items.iter_mut().filter_map(|item| {
        if let ImplItem::Method(method) = item {
            let vis = method.vis.clone();
            let sig = method.sig.clone();
            method.vis = Visibility::Inherited;
            Some((vis, sig))
        } else {
            None
        }
    }).collect()
}

fn redirect_method(vis: Visibility, mut sig: Signature, trait_: &Path) -> Result<ImplItem, Error> {
    let mut arg_count: usize = 0;
    let mut args = Vec::new();
    for arg in sig.inputs.iter_mut() {
        match arg {
            FnArg::Receiver(arg) => {
                if arg.reference.is_none() {
                    arg.mutability = None;
                }
                let ident = Ident::new("self", arg.span());
                args.push(ident);
            },
            FnArg::Typed(arg) => {
                match *arg.pat {
                    Pat::Ident(ref mut pat_ident) if pat_ident.ident == "self" => {
                        pat_ident.mutability = None;
                        args.push(pat_ident.ident.clone());
                    },
                    _ => {
                        arg_count += 1;
                        let ident = Ident::new(&format!("arg{}", arg_count), arg.span());
                        arg.pat = Box::new(Pat::Ident(PatIdent {
                            attrs: Vec::new(),
                            by_ref: None,
                            mutability: None,
                            ident: ident.clone(),
                            subpat: None,
                        }));
                        args.push(ident);
                    },
                }
            },
        }
    }
    let fn_name = &sig.ident;
    Ok(ImplItem::Verbatim(quote!(
        #[doc(hidden)]
        #[inline(always)]
        #vis #sig {
            <Self as #trait_>::#fn_name(#(#args),*)
        }
    )))
}
