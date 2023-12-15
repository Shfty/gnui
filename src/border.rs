use std::iter::Sum;

use clap::{ArgEnum, PossibleValue};
use tui::widgets::{BorderType as TuiBorderType, Borders};

#[derive(Debug, Copy, Clone, ArgEnum)]
pub enum Border {
    /// Show the top border
    Top,
    /// Show the right border
    Right,
    /// Show the bottom border
    Bottom,
    /// Show the left border
    Left,
    /// Show all borders
    All,
}

impl Sum<Border> for Borders {
    fn sum<I: Iterator<Item = Border>>(iter: I) -> Self {
        let mut out: Borders = Borders::empty();
        for modifier in iter {
            out |= match modifier {
                Border::Top => Borders::TOP,
                Border::Right => Borders::RIGHT,
                Border::Bottom => Borders::BOTTOM,
                Border::Left => Borders::LEFT,
                Border::All => Borders::ALL,
            };
        }
        out
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BorderType(TuiBorderType);

impl Default for BorderType {
    fn default() -> Self {
        BorderType(TuiBorderType::Plain)
    }
}

impl From<BorderType> for TuiBorderType {
    fn from(border_type: BorderType) -> Self {
        border_type.0
    }
}

impl ArgEnum for BorderType {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            BorderType(TuiBorderType::Plain),
            BorderType(TuiBorderType::Rounded),
            BorderType(TuiBorderType::Double),
            BorderType(TuiBorderType::Thick),
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue<'a>> {
        Some(match self.0 {
            TuiBorderType::Plain => PossibleValue::new("plain"),
            TuiBorderType::Rounded => PossibleValue::new("rounded"),
            TuiBorderType::Double => PossibleValue::new("double"),
            TuiBorderType::Thick => PossibleValue::new("thick"),
        })
    }
}
