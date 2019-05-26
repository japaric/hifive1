use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::{ast::App, Context};
use syn::Ident;

use crate::codegen::util;

pub fn codegen(scheduler: Context, name: &Ident, app: &App) -> TokenStream2 {
    let schedulee = &app.software_tasks[name];

    let fq = util::fq_ident(name);
    let tq = util::tq_ident();
    let (dequeue, enqueue) = if scheduler.is_init() {
        (quote!(#fq.dequeue()), quote!(#tq.enqueue_unchecked(nr);))
    } else {
        (
            quote!((#fq { priority }).lock(|fq| fq.split().1.dequeue())),
            quote!((#tq { priority }).lock(|tq| tq.enqueue_unchecked(nr));),
        )
    };

    let instants = util::instants_ident(name);
    let (_, tupled, _, _) = util::regroup_inputs(&schedulee.inputs);
    let inputs = util::inputs_ident(name);
    let t = util::schedule_t_ident();
    quote!(
        unsafe {
            use rtfm::Mutex as _;

            let input = #tupled;
            if let Some(index) = #dequeue {
                #inputs.get_unchecked_mut(usize::from(index)).as_mut_ptr().write(input);

                #instants.get_unchecked_mut(usize::from(index)).as_mut_ptr().write(instant);

                let nr = rtfm::export::NotReady {
                    instant,
                    index,
                    task: #t::#name,
                };

                #enqueue

                Ok(())
            } else {
                Err(input)
            }
        }
    )
}
