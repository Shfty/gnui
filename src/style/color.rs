use clap::{ArgEnum, PossibleValue};
use tui::style::Color as TuiColor;

#[derive(Debug, Copy, Clone)]
pub struct Color(TuiColor);

impl ArgEnum for Color {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Color(TuiColor::Reset),
            Color(TuiColor::Black),
            Color(TuiColor::Red),
            Color(TuiColor::Green),
            Color(TuiColor::Yellow),
            Color(TuiColor::Blue),
            Color(TuiColor::Magenta),
            Color(TuiColor::Cyan),
            Color(TuiColor::Gray),
            Color(TuiColor::DarkGray),
            Color(TuiColor::LightRed),
            Color(TuiColor::LightGreen),
            Color(TuiColor::LightYellow),
            Color(TuiColor::LightBlue),
            Color(TuiColor::LightMagenta),
            Color(TuiColor::LightCyan),
            Color(TuiColor::White),
            Color(TuiColor::Rgb(0, 0, 0)),
            Color(TuiColor::Indexed(0)),
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue<'a>> {
        Some(match self.0 {
            TuiColor::Reset => PossibleValue::new("reset"),
            TuiColor::Black => PossibleValue::new("black"),
            TuiColor::Red => PossibleValue::new("red"),
            TuiColor::Green => PossibleValue::new("green"),
            TuiColor::Yellow => PossibleValue::new("yellow"),
            TuiColor::Blue => PossibleValue::new("blue"),
            TuiColor::Magenta => PossibleValue::new("magenta"),
            TuiColor::Cyan => PossibleValue::new("cyan"),
            TuiColor::Gray => PossibleValue::new("gray"),
            TuiColor::DarkGray => PossibleValue::new("dark-gray"),
            TuiColor::LightRed => PossibleValue::new("light-red"),
            TuiColor::LightGreen => PossibleValue::new("light-green"),
            TuiColor::LightYellow => PossibleValue::new("light-yellow"),
            TuiColor::LightBlue => PossibleValue::new("light-blue"),
            TuiColor::LightMagenta => PossibleValue::new("light-magenta"),
            TuiColor::LightCyan => PossibleValue::new("light-cyan"),
            TuiColor::White => PossibleValue::new("white"),
            TuiColor::Rgb(_, _, _) => PossibleValue::new("<R,G,B>"),
            TuiColor::Indexed(_) => PossibleValue::new("<INDEX>"),
        })
    }

    fn from_str(input: &str, ignore_case: bool) -> std::result::Result<Self, String> {
        Self::value_variants()
            .iter()
            .find(|v| { v.to_possible_value()
                    .expect("ArgEnum::value_variants contains only values with a corresponding ArgEnum::to_possible_value")
                    .matches(input, ignore_case)
            })
            .cloned()
            .ok_or_else(|| format!("Invalid variant: {}", input))
    }
}

impl From<Color> for TuiColor {
    fn from(color: Color) -> Self {
        color.0
    }
}

