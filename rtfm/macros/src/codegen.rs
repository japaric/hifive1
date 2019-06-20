use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::{analyze::Analysis, ast::App};

mod assertions;
mod dispatcher;
mod hardware_tasks;
mod idle;
mod init;
mod locals;
mod module;
mod post_init;
mod pre_init;
mod resources;
mod resources_struct;
mod schedule;
mod schedule_body;
mod software_tasks;
mod spawn;
mod spawn_body;
mod timer_queue;
mod util;

pub fn app(app: &App, analysis: &Analysis) -> TokenStream2 {
    let assertion_stmts = assertions::codegen(analysis);

    let pre_init_stmts = pre_init::codegen(&app, analysis);

    let (const_app_init, root_init, user_init, call_init) = init::codegen(app, analysis);

    let post_init_stmts = post_init::codegen(app, analysis);

    let (const_app_idle, root_idle, user_idle, call_idle) = idle::codegen(app, analysis);

    let (const_app_resources, root_resources) = resources::codegen(app, analysis);

    let (const_app_hardware_tasks, root_hardware_tasks, user_hardware_tasks) =
        hardware_tasks::codegen(app, analysis);

    let (const_app_software_tasks, root_software_tasks, user_software_tasks) =
        software_tasks::codegen(app, analysis);

    let const_app_dispatcher = dispatcher::codegen(app, analysis);

    let const_app_spawn = spawn::codegen(app);

    let const_app_timer_queue = timer_queue::codegen(app, analysis);

    let const_app_schedule = schedule::codegen(app);

    let name = &app.name;
    quote!(
        #user_init

        #user_idle

        #(#user_hardware_tasks)*

        #(#user_software_tasks)*

        #(#root_init)*

        #(#root_idle)*

        #(#root_resources)*

        #(#root_hardware_tasks)*

        #(#root_software_tasks)*

        /// Implementation details
        const #name: () = {
            #(#const_app_init)*

            #(#const_app_idle)*

            #(#const_app_resources)*

            #(#const_app_hardware_tasks)*

            #(#const_app_software_tasks)*

            #(#const_app_dispatcher)*

            #(#const_app_spawn)*

            #(#const_app_timer_queue)*

            #(#const_app_schedule)*

            #[no_mangle]
            unsafe extern "C" fn main() -> ! {
                #(#assertion_stmts)*

                #(#pre_init_stmts)*

                #call_init

                #(#post_init_stmts)*

                #call_idle
            }
        };
    )
}
