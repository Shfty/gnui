use clap::ArgEnum;
use tui::layout::Alignment as TuiAlignment;

#[derive(Debug, Copy, Clone, ArgEnum)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

impl From<Alignment> for TuiAlignment {
    fn from(alignment: Alignment) -> Self {
        match alignment {
            Alignment::Left => TuiAlignment::Left,
            Alignment::Center => TuiAlignment::Center,
            Alignment::Right => TuiAlignment::Right,
        }
    }
}
