use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::{analyze::Analysis, ast::App};

use crate::codegen::util;

/// Generates the task dispatcher
pub fn codegen(app: &App, analysis: &Analysis) -> Vec<TokenStream2> {
    // single-core mode
    let core = 0;

    let mut items = vec![];

    for (&_receiver, dispatchers) in &analysis.channels {
        for (&level, channels) in dispatchers {
            let mut stmts = vec![];

            for (&_sender, channel) in channels {
                let variants = channel
                    .tasks
                    .iter()
                    .map(|name| {
                        let cfgs = &app.software_tasks[name].cfgs;

                        quote!(
                            #(#cfgs)*
                            #name
                        )
                    })
                    .collect::<Vec<_>>();

                let t = util::spawn_t_ident(level);
                items.push(quote!(
                    /// All the software tasks
                    #[allow(non_camel_case_types)]
                    #[derive(Clone, Copy)]
                    enum #t {
                        #(#variants,)*
                    }
                ));

                let n = util::capacity_typenum(channel.capacity, true);
                let rq = util::rq_ident(level);
                let rq_ty = quote!(rtfm::export::SCRQ<#t, #n>);
                let rq_expr = quote!(rtfm::export::Queue(unsafe {
                    rtfm::export::iQueue::u8_sc()
                }));
                items.push(quote!(
                    /// Queue of tasks ready to be dispatched
                    static mut #rq: #rq_ty = #rq_expr;
                ));

                if let Some(ceiling) = channel.ceiling {
                    items.push(quote!(
                        struct #rq<'a> {
                            priority: &'a rtfm::export::Priority,
                        }
                    ));

                    items.push(util::impl_mutex(
                        &[],
                        false,
                        &rq,
                        rq_ty,
                        ceiling,
                        quote!(&mut #rq),
                        app,
                        analysis,
                    ));
                }

                let arms = channel
                    .tasks
                    .iter()
                    .map(|name| {
                        let task = &app.software_tasks[name];
                        let cfgs = &task.cfgs;
                        let fq = util::fq_ident(name);
                        let inputs = util::inputs_ident(name);
                        let (_, tupled, pats, _) = util::regroup_inputs(&task.inputs);

                        let (let_instant, instant) = if app.uses_schedule(core) {
                            let instants = util::instants_ident(name);

                            (
                                quote!(
                                    let instant =
                                        #instants.get_unchecked(usize::from(index)).as_ptr().read();
                                ),
                                quote!(, instant),
                            )
                        } else {
                            (quote!(), quote!())
                        };

                        let locals_new = if task.locals.is_empty() {
                            quote!()
                        } else {
                            quote!(#name::Locals::new(),)
                        };

                        quote!(
                            #(#cfgs)*
                            #t::#name => {
                                let #tupled =
                                    #inputs.get_unchecked(usize::from(index)).as_ptr().read();
                                #let_instant
                                #fq.split().0.enqueue_unchecked(index);
                                let priority = &rtfm::export::Priority::new(PRIORITY);
                                #name(
                                    #locals_new
                                    #name::Context::new(priority #instant)
                                    #(,#pats)*
                                )
                            }
                        )
                    })
                    .collect::<Vec<_>>();

                stmts.push(quote!(
                    while let Some((task, index)) = #rq.split().1.dequeue() {
                        match task {
                            #(#arms)*
                        }
                    }
                ));
            }

            let (prologue, epilogue) = if app.hardware_tasks.is_empty() {
                (None, None)
            } else {
                // if there are hardware tasks then we re-enable interrupt preemption so interrupts
                // can preempt software tasks

                let mask = if analysis.timer_queues.is_empty() {
                    quote!(rtfm::export::mie::MSIE)
                } else {
                    quote!(rtfm::export::mie::MSIE | rtfm::export::mie::MTIE)
                };

                (
                    Some(quote!(
                        // mask software (and timer) interrupt(s)
                        rtfm::export::mie::clear(#mask);

                        // enable interrupt preemption
                        rtfm::export::mstatus::set_mie();
                    )),
                    Some(quote!(
                        // disable interrupt preemption
                        rtfm::export::mstatus::clear_mie();

                        // unmask software (and timer) interrupt(s)
                        rtfm::export::mie::set(#mask);
                    )),
                )
            };

            debug_assert_eq!(level, 1);
            items.push(quote!(
                /// Interrupt handler used to dispatch tasks
                #[allow(non_snake_case)]
                #[no_mangle]
                unsafe extern "C" fn msi() {
                    /// The priority of this interrupt handler
                    const PRIORITY: u8 = 1;

                    rtfm::export::msip::clear_msi();

                    #prologue

                    #(#stmts)*

                    #epilogue
                }
            ));
        }
    }

    items
}
