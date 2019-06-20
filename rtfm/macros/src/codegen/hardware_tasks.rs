use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::{analyze::Analysis, ast::App, Context};

use crate::codegen::{locals, module, resources_struct};

/// Generate support code for hardware tasks (`#[exception]`s and `#[interrupt]`s)
pub fn codegen(
    app: &App,
    analysis: &Analysis,
) -> (
    // const_app_hardware_tasks -- interrupt handlers and `${task}Resources` constructors
    Vec<TokenStream2>,
    // root_hardware_tasks -- items that must be placed in the root of the crate:
    // - `${task}Locals` structs
    // - `${task}Resources` structs
    // - `${task}` modules
    Vec<TokenStream2>,
    // user_hardware_tasks -- the `#[task]` functions written by the user
    Vec<TokenStream2>,
) {
    let mut const_app = vec![];
    let mut root = vec![];
    let mut user_tasks = vec![];

    // single-core mode
    let core = 0;

    for (name, task) in &app.hardware_tasks {
        let (let_instant, instant) = if app.uses_schedule(core) {
            (
                Some(quote!(let instant = rtfm::Instant::now();)),
                Some(quote!(, instant)),
            )
        } else {
            (None, None)
        };

        let locals_new = if task.locals.is_empty() {
            quote!()
        } else {
            quote!(#name::Locals::new(),)
        };

        let symbol = &task.args.binds;

        let plic_priority = task.args.priority - if app.software_tasks.is_empty() { 0 } else { 1 };

        let mut prologue = vec![];
        let mut epilogue = vec![];

        // raise the threshold during the execution of this external interrupt so we don't get
        // preempted by lower priority interrupts
        if plic_priority == 1 {
            prologue.push(quote!(
                let current = 0;
            ));
        } else {
            prologue.push(quote!(
                let current = rtfm::export::plic::get_threshold();
            ));
        }

        prologue.push(quote!(
            rtfm::export::plic::set_threshold(#plic_priority);
        ));

        if !app.software_tasks.is_empty() {
            // mask software (and timer) interrupts during the execution of external interrupts

            let mut mask = vec![quote!(rtfm::export::mie::MSIE)];
            if analysis.timer_queues.is_empty() {
                mask.push(quote!(rtfm::export::mie::MTIE));
            }

            let mask = &mask;
            prologue.push(quote!(
                rtfm::export::mie::clear(#(#mask)|*);

                // enable preemption
                rtfm::export::mstatus::set_mie();
            ));

            epilogue.push(quote!(
                // disable preemption
                rtfm::export::mstatus::clear_mie();

                rtfm::export::mie::clear(#(#mask)|*);
            ));
        }

        epilogue.push(quote!(
            rtfm::export::plic::set_threshold(current);
        ));

        let priority = task.args.priority;
        const_app.push(quote!(
            #[allow(non_snake_case)]
            #[no_mangle]
            unsafe extern "C" fn #symbol() {
                const PRIORITY: u8 = #priority;

                #let_instant

                #(#prologue)*

                crate::#name(
                    #locals_new
                    #name::Context::new(&rtfm::export::Priority::new(PRIORITY) #instant)
                );

                #(#epilogue)*
            }
        ));

        let mut needs_lt = false;

        // `${task}Resources`
        if !task.args.resources.is_empty() {
            let (item, constructor) = resources_struct::codegen(
                Context::HardwareTask(name),
                priority,
                &mut needs_lt,
                app,
                analysis,
            );

            root.push(item);

            const_app.push(constructor);
        }

        root.push(module::codegen(Context::HardwareTask(name), needs_lt, app));

        // `${task}Locals`
        let mut locals_pat = None;
        if !task.locals.is_empty() {
            let (struct_, pat) = locals::codegen(Context::HardwareTask(name), &task.locals, app);

            root.push(struct_);
            locals_pat = Some(pat);
        }

        let attrs = &task.attrs;
        let context = &task.context;
        let stmts = &task.stmts;
        user_tasks.push(quote!(
            #(#attrs)*
            #[allow(non_snake_case)]
            fn #name(#(#locals_pat,)* #context: #name::Context) {
                use rtfm::Mutex as _;

                #(#stmts)*
            }
        ));
    }

    (const_app, root, user_tasks)
}
