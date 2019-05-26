use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::{
    analyze::Analysis,
    ast::{App, HardwareTaskKind},
};

use crate::codegen::util;

/// Generates code that runs before `#[init]`
pub fn codegen(app: &App, analysis: &Analysis) -> Vec<TokenStream2> {
    let mut stmts = vec![];

    // populate the `FreeQueue`s
    for (name, senders) in &analysis.free_queues {
        let task = &app.software_tasks[name];
        let cap = task.args.capacity;

        let fq = util::fq_ident(name);

        stmts.push(quote!(
            (0..#cap).for_each(|i| #fq.enqueue_unchecked(i));
        ));
    }

    // unmask interrupts and set their priorities
    for (name, task) in app.hardware_tasks.iter() {
        let interrupt = task.args.binds(name);
        let priority = task.args.priority - if app.software_tasks.is_empty() { 0 } else { 1 };

        stmts.push(quote!(
            rtfm::export::plic::set_priority(rtfm::export::Interrupt::#interrupt, #priority);
            rtfm::export::plic::enable(rtfm::export::Interrupt::#interrupt);
        ));
    }

    // `msip` is not cleared across software resets to clear it enough to prevent spurious software
    // interrupts
    if !app.software_tasks.is_empty() {
        stmts.push(quote!(
            rtfm::export::msip::clear_msi();
        ));
    }

    // `mtimecmp` is preserved across resets so write the max value to prevent spurious
    // timer interrupts
    if !analysis.timer_queues.is_empty() {
        stmts.push(quote!(
            rtfm::export::mtimecmp::write_max();
        ));
    }

    stmts
}
