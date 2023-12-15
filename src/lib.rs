// TODO: Prevent event and input thread panics when main exits
// TODO: Finish chart implementation
// TODO: Investigate using serde to deserialize input
//       * Could provide free support for various formats
// TODO: Investigate using a second input stream for runtime commands
//       * ex. Paragraph scrolling

use backend::Backend;
use threads::main::Terminal;
use tui::backend::CrosstermBackend;

pub mod alignment;
pub mod backend;
pub mod border;
pub mod path;
pub mod style;
pub mod threads;
pub mod widgets;

pub type Result = std::result::Result<(), std::io::Error>;

pub fn terminal<T: Into<Backend>>(backend: T) -> std::io::Result<Terminal> {
    Terminal::new(CrosstermBackend::new(backend.into()))
}
