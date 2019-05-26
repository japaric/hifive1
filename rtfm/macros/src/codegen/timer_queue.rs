use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::{analyze::Analysis, ast::App};

use crate::codegen::util;

/// Generates timer queues and timer queue handlers
pub fn codegen(app: &App, analysis: &Analysis) -> Vec<TokenStream2> {
    let mut items = vec![];

    for (&_sender, timer_queue) in &analysis.timer_queues {
        // single-core mode
        debug_assert_eq!(_sender, 0);

        let t = util::schedule_t_ident();

        // Enumeration of `schedule`-able tasks
        {
            let variants = timer_queue
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

            items.push(quote!(
                /// `schedule`-able tasks
                #[allow(non_camel_case_types)]
                #[derive(Clone, Copy)]
                enum #t {
                    #(#variants,)*
                }
            ));
        }

        let tq = util::tq_ident();

        // Static variable and resource proxy
        {
            let n = util::capacity_typenum(timer_queue.capacity, false);
            let tq_ty = quote!(rtfm::export::TimerQueue<#t, #n>);

            items.push(quote!(
                /// Timer queue
                static mut #tq: #tq_ty = rtfm::export::TimerQueue(
                    rtfm::export::BinaryHeap(
                        rtfm::export::iBinaryHeap::new()
                    )
                );

                struct #tq<'a> {
                    priority: &'a rtfm::export::Priority,
                }
            ));

            items.push(util::impl_mutex(
                &[],
                false,
                &tq,
                tq_ty,
                timer_queue.ceiling,
                quote!(&mut #tq),
                app,
                analysis,
            ));
        }

        // Timer queue handler
        {
            let arms = timer_queue
                .tasks
                .iter()
                .map(|name| {
                    let task = &app.software_tasks[name];

                    let cfgs = &task.cfgs;
                    let priority = task.args.priority;
                    let receiver = task.args.core;
                    let rq = util::rq_ident(priority);
                    let rqt = util::spawn_t_ident(priority);

                    quote!(
                        #(#cfgs)*
                        #t::#name => {
                            (#rq { priority: &rtfm::export::Priority::new(PRIORITY) }).lock(|rq| {
                                rq.split().0.enqueue_unchecked((#rqt::#name, index))
                            });

                            rtfm::export::msip::set_msi();
                        }
                    )
                })
                .collect::<Vec<_>>();

            let (prologue, epilogue) = if app.hardware_tasks.is_empty() {
                (None, None)
            } else {
                // if there are hardware tasks then we re-enable interrupt preemption so interrupts
                // can preempt software tasks

                (
                    Some(quote!(
                        // mask software (and timer) interrupt(s)
                        rtfm::export::mie::clear(rtfm::export::mie::MSIE | rtfm::export::mie::MTIE);

                        // enable interrupt preemption
                        rtfm::export::mstatus::set_mie();
                    )),
                    Some(quote!(
                        // disable interrupt preemption
                        rtfm::export::mstatus::clear_mie();

                        // unmask software (and timer) interrupt(s)
                        rtfm::export::mie::set(rtfm::export::mie::MSIE | rtfm::export::mie::MTIE);
                    )),
                )
            };

            let priority = timer_queue.priority;
            items.push(quote!(
                #[no_mangle]
                unsafe extern "C" fn mti() {
                    use rtfm::Mutex as _;

                    /// The priority of this handler
                    const PRIORITY: u8 = #priority;

                    #prologue

                    while let Some((task, index)) = (#tq {
                        // NOTE dynamic priority is always the static priority at this point
                        priority: &rtfm::export::Priority::new(PRIORITY),
                    })
                    // NOTE `inline(always)` produces faster and smaller code
                        .lock(#[inline(always)]
                                |tq| tq.dequeue())
                    {
                        match task {
                            #(#arms)*
                        }
                    }

                    #epilogue
                }
            ));
        }
    }

    items
}
