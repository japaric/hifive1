use std::collections::{BTreeMap, HashSet};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::ast::App;

use crate::codegen::{spawn_body, util};

/// Generates all `${ctxt}::Spawn` methods
pub fn codegen(app: &App) -> Vec<TokenStream2> {
    let mut items = vec![];

    let mut seen = BTreeMap::<u8, HashSet<_>>::new();
    for (spawner, spawnees) in app.spawn_callers() {
        let sender = spawner.core(app);
        let seen = seen.entry(sender).or_default();
        let mut methods = vec![];

        for name in spawnees {
            let spawnee = &app.software_tasks[name];
            let receiver = spawnee.args.core;
            let cfgs = &spawnee.cfgs;
            let (args, _, untupled, ty) = util::regroup_inputs(&spawnee.inputs);
            let args = &args;

            if spawner.is_init() {
                // `init` uses a special spawn implementation; it doesn't use the `spawn_${name}`
                // functions which are shared by other contexts

                let body = spawn_body::codegen(spawner, &name, app);

                let let_instant = if app.uses_schedule(receiver) {
                    Some(quote!(let instant = unsafe { rtfm::Instant::artificial(0) };))
                } else {
                    None
                };

                methods.push(quote!(
                    #(#cfgs)*
                    fn #name(&self #(,#args)*) -> Result<(), #ty> {
                        #let_instant
                        #body
                    }
                ));
            } else {
                let spawn = util::spawn_ident(name);

                if !seen.contains(name) {
                    // generate a `spawn_${name}_S${sender}` function
                    seen.insert(name);

                    let instant = if app.uses_schedule(receiver) {
                        Some(quote!(, instant: rtfm::Instant))
                    } else {
                        None
                    };

                    let body = spawn_body::codegen(spawner, &name, app);

                    items.push(quote!(
                        #(#cfgs)*
                        unsafe fn #spawn(
                            priority: &rtfm::export::Priority
                            #instant
                            #(,#args)*
                        ) -> Result<(), #ty> {
                            #body
                        }
                    ));
                }

                let (let_instant, instant) = if app.uses_schedule(receiver) {
                    (
                        Some(if spawner.is_idle() {
                            quote!(let instant = rtfm::Instant::now();)
                        } else {
                            quote!(let instant = self.instant();)
                        }),
                        Some(quote!(, instant)),
                    )
                } else {
                    (None, None)
                };

                methods.push(quote!(
                    #(#cfgs)*
                    #[inline(always)]
                    fn #name(&self #(,#args)*) -> Result<(), #ty> {
                        unsafe {
                            #let_instant
                            #spawn(self.priority() #instant #(,#untupled)*)
                        }
                    }
                ));
            }
        }

        let lt = if spawner.is_init() {
            None
        } else {
            Some(quote!('a))
        };

        let spawner = spawner.ident(app);
        debug_assert!(!methods.is_empty());
        items.push(quote!(
            impl<#lt> #spawner::Spawn<#lt> {
                #(#methods)*
            }
        ));
    }

    items
}
