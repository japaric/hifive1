#![deny(warnings)]
#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use std::{fs, path::Path};

use rtfm_syntax::Settings;

mod check;
mod codegen;

#[proc_macro_attribute]
pub fn app(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut settings = Settings::default();
    settings.parse_extern_interrupt = true;
    settings.parse_binds = true;
    settings.parse_schedule = true;
    settings.optimize_priorities = true;

    let (app, analysis) = match rtfm_syntax::parse(args, input, settings) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    if let Err(e) = check::app(&app) {
        return e.to_compile_error().into();
    };

    let ts = codegen::app(&app, &analysis);

    // Try to write the expanded code to disk
    if Path::new("target").exists() {
        fs::write("target/rtfm-expansion.rs", ts.to_string()).ok();
    }

    ts.into()
}
