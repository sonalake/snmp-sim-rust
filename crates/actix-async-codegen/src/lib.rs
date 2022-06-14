use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

/// Enables an async main function in tokio's [`Localset`](tokio::task::LocalSet).
///
/// # Examples
///
/// ```ignore
/// #[actix_async::main]
/// async fn main() -> std::io::Result<()> {
///     Ok(())
/// }
/// ```
///
/// # Expend
///
/// macro would expend into following code
/// ```ignore
/// fn main() -> std::io::Result<()> {
///     tokio::runtime::Builder::new_current_thread()
///         .enable_all()
///         .build()
///         .unwrap()
///         .block_on(tokio::task::LocalSet::new().run_until(async {
///             Ok(())        
///         }))
/// }
/// ```
#[cfg(not(test))]
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;

    if name != "main" {
        return TokenStream::from(quote_spanned! { name.span() =>
            compile_error!("only the main function can be tagged with #[actix_async::main]"),
        });
    }

    if input.sig.asyncness.is_none() {
        return TokenStream::from(quote_spanned! { input.span() =>
            compile_error!("the async keyword is missing from the function declaration"),
        });
    }

    let result = quote! {
        #vis fn main() #ret {
            #(#attrs)*
            async fn main(#inputs) #ret {
                #body
            }

            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(tokio::task::LocalSet::new().run_until(async {
                    main().await
                }))
        }

    };

    result.into()
}

/// Enables an async test function in tokio's [`Localset`](tokio::task::LocalSet).
///
/// # Examples
///
/// ```ignore
/// #[actix_async::test]
/// async fn my_test() -> std::io::Result<()> {
///     assert_eq!(2 * 2, 4);
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;

    if input.sig.asyncness.is_none() {
        return TokenStream::from(quote_spanned! { input.span() =>
            compile_error!("the async keyword is missing from the function declaration"),
        });
    }

    let result = quote! {
        #[::core::prelude::v1::test]
        #(#attrs)*
        #vis fn #name() #ret {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(tokio::task::LocalSet::new().run_until(async {
                    #body
                }))
        }
    };

    result.into()
}

/// Enables async method in [Handler](actix_async::prelude::Handler) trait.
///
/// # Examples
///
/// ```ignore
/// #[actix_async::handler]
/// impl Handler<Msg> for Actor {
///     async fn handle(&self, msg: Msg, ctx: Context<'_, Self>) {}
/// }
/// ```
/// # Expend
///
/// macro would expend into following code
/// ```ignore
/// #[async_trait::async_trait(?Send)]
/// impl Handler<Msg> for Actor {
///     async fn handle(&self, msg: Msg, ctx: Context<'_, Self>) {}
/// }
/// ```
#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemImpl);

    let result = quote! {
        #[actix_async::__async_trait(?Send)]
        #input
    };

    result.into()
}
