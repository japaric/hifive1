// #![deny(warnings)]
#![allow(warnings)]
#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use std::{fs, path::Path};

use rtfm_syntax::Settings;

mod codegen;
mod check;

#[proc_macro_attribute]
pub fn app(args: TokenStream, input: TokenStream) -> TokenStream {
    let (app, analysis) = match rtfm_syntax::parse(
        args,
        input,
        Settings {
            parse_extern_interrupt: true,
            parse_interrupt: true,
            parse_schedule: true,
            optimize_priorities: true,
            ..Settings::default()
        },
    ) {
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
