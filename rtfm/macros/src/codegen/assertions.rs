use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::analyze::Analysis;

/// Generates compile-time assertions that check that types implement the `Send` / `Sync` traits
pub fn codegen(analysis: &Analysis) -> Vec<TokenStream2> {
    let mut stmts = vec![];

    if let Some(types) = analysis.send_types.get(&0) {
        for ty in types {
            stmts.push(quote!(rtfm::export::assert_send::<#ty>();));
        }
    }

    if let Some(types) = analysis.sync_types.get(&0) {
        for ty in types {
            stmts.push(quote!(rtfm::export::assert_sync::<#ty>();));
        }
    }

    stmts
}
