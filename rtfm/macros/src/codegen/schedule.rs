use std::collections::{BTreeMap, HashSet};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::ast::App;

use crate::codegen::{schedule_body, util};

/// Generates all `${ctxt}::Schedule` methods
pub fn codegen(app: &App) -> Vec<TokenStream2> {
    let mut items = vec![];

    let mut seen = HashSet::new();
    for (scheduler, schedulees) in app.schedule_callers() {
        let instant = quote!(rtfm::Instant);

        let mut methods = vec![];

        for name in schedulees {
            let schedulee = &app.software_tasks[name];
            let cfgs = &schedulee.cfgs;
            let (args, _, untupled, ty) = util::regroup_inputs(&schedulee.inputs);
            let args = &args;

            if scheduler.is_init() {
                // `init` uses a special `schedule` implementation; it doesn't use the
                // `schedule_${name}` functions which are shared by other contexts

                let body = schedule_body::codegen(scheduler, &name, app);

                methods.push(quote!(
                    #(#cfgs)*
                    fn #name(&self, instant: #instant #(,#args)*) -> Result<(), #ty> {
                        #body
                    }
                ));
            } else {
                let schedule = util::schedule_ident(name);

                if !seen.contains(name) {
                    // generate a `schedule_${name}` function
                    seen.insert(name);

                    let body = schedule_body::codegen(scheduler, &name, app);

                    items.push(quote!(
                        #(#cfgs)*
                        unsafe fn #schedule(
                            priority: &rtfm::export::Priority,
                            instant: #instant
                            #(,#args)*
                        ) -> Result<(), #ty> {
                            #body
                        }
                    ));
                }

                methods.push(quote!(
                    #(#cfgs)*
                    #[inline(always)]
                    fn #name(&self, instant: #instant #(,#args)*) -> Result<(), #ty> {
                        unsafe {
                            #schedule(self.priority(), instant #(,#untupled)*)
                        }
                    }
                ));
            }
        }

        let lt = if scheduler.is_init() {
            None
        } else {
            Some(quote!('a))
        };

        let scheduler = scheduler.ident(app);
        debug_assert!(!methods.is_empty());
        items.push(quote!(
            impl<#lt> #scheduler::Schedule<#lt> {
                #(#methods)*
            }
        ));
    }

    items
}
