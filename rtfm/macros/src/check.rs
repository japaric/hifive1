use proc_macro2::Span as Span2;
use rtfm_syntax::ast::App;
use syn::parse;

// Software interrupts and timer interrupts bypass the interrupt prioritization mechanism so we'll
// enforce here that they'll be given the lowest priority
pub fn app(app: &App) -> parse::Result<()> {
    // check that all software tasks have the same priority
    let mut priority = None;
    if !app.software_tasks.values().all(|task| {
        if let Some(prio) = priority {
            prio == task.args.priority
        } else {
            priority = Some(task.args.priority);
            true
        }
    }) {
        return Err(parse::Error::new(
            Span2::call_site(),
            "all software tasks must be given the same priority",
        ));
    }

    if let Some(priority) = priority {
        if !app
            .hardware_tasks
            .values()
            .all(|task| priority < task.args.priority)
        {
            return Err(parse::Error::new(
                Span2::call_site(),
                "all hardware tasks must have higher priority than software tasks",
            ));
        }
    }

    // check that priorities don't exceed the device supported maximum
    const MAX: u8 = 7;
    if app
        .hardware_tasks
        .values()
        .map(|task| task.args.priority)
        .max()
        .unwrap_or(0)
        > MAX + if app.software_tasks.is_empty() { 0 } else { 1 }
    {
        return Err(parse::Error::new(
            Span2::call_site(),
            &format!(
                "hardware tasks can't span more than {} priority levels",
                MAX
            ),
        ));
    }

    Ok(())
}
