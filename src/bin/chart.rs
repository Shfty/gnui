use gnui::{
    backend::{backend_from_os_str, Backend},
    terminal,
    threads::input::InputThread,
    threads::{
        event::event_thread,
        main::{main_thread, InputBuffer},
    },
    widgets::chart::Chart,
    Result,
};

use clap::Parser;

/// Display text input as a TUI chart
#[derive(Debug, Parser)]
#[clap(name = "chart", author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    input_thread: InputThread,

    /// Output file; if -, write to standard output
    #[clap(parse(try_from_os_str = backend_from_os_str), default_value = "-")]
    output: Backend,

    #[clap(flatten)]
    chart: Chart,
}

fn main() -> Result {
    let Cli {
        input_thread,
        output,
        chart,
    } = Cli::parse();

    let buf: InputBuffer = Default::default();

    main_thread(
        &mut terminal(output)?,
        input_thread.spawn(),
        event_thread(),
        buf.clone(),
        chart.draw(buf),
    )
}
