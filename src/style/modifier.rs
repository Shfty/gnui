use std::iter::Sum;

use clap::ArgEnum;
use tui::style::Modifier as TuiModifier;

#[derive(Debug, Copy, Clone, ArgEnum)]
pub enum Modifier {
    Bold,
    Dim,
    Italic,
    Underlined,
    SlowBlink,
    RapidBlink,
    Reversed,
    Hidden,
    CrossedOut,
}

impl Sum<Modifier> for TuiModifier {
    fn sum<I: Iterator<Item = Modifier>>(iter: I) -> Self {
        let mut out: TuiModifier = TuiModifier::empty();
        for modifier in iter {
            out |= match modifier {
                Modifier::Bold => TuiModifier::BOLD,
                Modifier::Dim => TuiModifier::DIM,
                Modifier::Italic => TuiModifier::ITALIC,
                Modifier::Underlined => TuiModifier::UNDERLINED,
                Modifier::SlowBlink => TuiModifier::SLOW_BLINK,
                Modifier::RapidBlink => TuiModifier::RAPID_BLINK,
                Modifier::Reversed => TuiModifier::REVERSED,
                Modifier::Hidden => TuiModifier::HIDDEN,
                Modifier::CrossedOut => TuiModifier::CROSSED_OUT,
            };
        }
        out
    }
}

