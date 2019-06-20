use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::{analyze::Analysis, ast::App};

/// Generates code that runs after `#[init]` returns
pub fn codegen(app: &App, analysis: &Analysis) -> Vec<TokenStream2> {
    // single-core mode
    let core = 0;

    let mut stmts = vec![];

    // initialize late resources
    if let Some(late_resources) = analysis.late_resources.get(&core) {
        for name in late_resources {
            // if it's live
            if analysis.locations.get(name).is_some() {
                stmts.push(quote!(#name.as_mut_ptr().write(late.#name);));
            }
        }
    }

    let mut mask = vec![];
    if !app.hardware_tasks.is_empty() {
        mask.push(quote!(rtfm::export::mie::MEIE));

        stmts.push(quote!(
            rtfm::export::plic::set_threshold(0);
        ));
    }

    if !app.software_tasks.is_empty() {
        mask.push(quote!(rtfm::export::mie::MSIE));
    }

    if !analysis.timer_queues.is_empty() {
        mask.push(quote!(rtfm::export::mie::MTIE));
    }

    // unmask the exceptions that we are going to use
    if !mask.is_empty() {
        stmts.push(quote!(rtfm::export::mie::write(#(#mask)|*);));
    }

    // reset the monotonic timer
    // since not long should have passed since reset writing the lower 32-bit word should be enough
    if !analysis.timer_queues.is_empty() {
        stmts.push(quote!(
            rtfm::export::mtimel::write(0);
        ));
    }

    // globally enable all interrupts -- this completes the `init`-ialization phase
    stmts.push(quote!(rtfm::export::mstatus::set_mie();));

    stmts
}
