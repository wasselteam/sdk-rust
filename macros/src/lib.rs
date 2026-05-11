use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(item as ItemFn);

    let name = &func.sig.ident;

    let expanded = quote! {
        #func

        struct __Plugin;

        impl wassel_sdk::bindings::exports::wassel::foundation::http_handler::Guest for __Plugin {
            fn handle_request(
                request: wassel_sdk::bindings::exports::wassel::foundation::http_handler::IncomingRequest,
                response_out: wassel_sdk::bindings::exports::wassel::foundation::http_handler::ResponseOutparam,
            ) {
                wassel_sdk::http::handle_request_with_handler(request, response_out, #name );
            }
        }

        wassel_sdk::export!(__Plugin);
    };

    TokenStream::from(expanded)
}
