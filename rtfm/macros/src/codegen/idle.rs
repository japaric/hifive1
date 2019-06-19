use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::{analyze::Analysis, ast::App, Context};

use crate::codegen::{locals, module, resources_struct, util};

/// Generates support code for `#[idle]` functions
pub fn codegen(
    app: &App,
    analysis: &Analysis,
) -> (
    // const_app_idle -- the `${idle}Resources` constructor
    Option<TokenStream2>,
    // root_idle -- items that must be placed in the root of the crate:
    // - the `${idle}Locals` struct
    // - the `${idle}Resources` struct
    // - the `${idle}` module, which contains types like `${idle}::Context`
    Vec<TokenStream2>,
    // user_idle
    Option<TokenStream2>,
    // call_idle
    TokenStream2,
) {
    // single-core mode
    let core = 0;

    if let Some(idle) = app.idles.get(&core) {
        let mut needs_lt = false;
        let mut const_app = None;
        let mut root_idle = vec![];
        let mut locals_pat = None;
        let mut locals_new = None;

        if !idle.args.resources.is_empty() {
            let (item, constructor) =
                resources_struct::codegen(Context::Idle(core), 0, &mut needs_lt, app, analysis);

            root_idle.push(item);
            const_app = Some(constructor);
        }

        let name = &idle.name;
        if !idle.locals.is_empty() {
            let (locals, pat) = locals::codegen(Context::Idle(core), &idle.locals, app);

            locals_new = Some(quote!(#name::Locals::new()));
            locals_pat = Some(pat);
            root_idle.push(locals);
        }

        root_idle.push(module::codegen(Context::Idle(core), needs_lt, app));

        let attrs = &idle.attrs;
        let context = &idle.context;
        let stmts = &idle.stmts;
        let user_idle = Some(quote!(
            #(#attrs)*
            #[allow(non_snake_case)]
            fn #name(#(#locals_pat,)* #context: #name::Context) -> ! {
                use rtfm::Mutex as _;

                #(#stmts)*
            }
        ));

        let call_idle = quote!(#name(
            #(#locals_new,)*
            #name::Context::new(&rtfm::export::Priority::new(0))
        ));

        (const_app, root_idle, user_idle, call_idle)
    } else {
        (
            None,
            vec![],
            None,
            quote!(loop {
                rtfm::export::asm::wfi();
            }),
        )
    }
}
