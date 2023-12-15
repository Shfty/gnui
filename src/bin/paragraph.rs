use gnui::{
    backend::{backend_from_os_str, Backend},
    terminal,
    threads::main::InputBuffer,
    threads::{event::event_thread, input::InputThread, main::main_thread},
    widgets::paragraph::Paragraph,
    Result,
};

use clap::Parser;

/// Display text input as a TUI paragraph
#[derive(Debug, Parser)]
#[clap(name = "paragraph", author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    input_thread: InputThread,

    /// Output file; if -, write to standard output
    #[clap(parse(try_from_os_str = backend_from_os_str), default_value = "-")]
    output: Backend,

    #[clap(flatten)]
    paragraph: Paragraph,
}

fn main() -> Result {
    let Cli {
        input_thread,
        output,
        paragraph,
    } = Cli::parse();

    let buf: InputBuffer = Default::default();

    main_thread(
        &mut terminal(output)?,
        input_thread.spawn(),
        event_thread(),
        buf.clone(),
        paragraph.draw(buf),
    )
}
