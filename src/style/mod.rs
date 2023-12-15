mod color;
mod modifier;

pub use color::Color;
pub use modifier::Modifier;

#[macro_export]
macro_rules ! Style {
    (
        $struct:ident,
        $help_heading:expr,
        $upper_fg:expr,
        $upper_bg:expr,
        $upper_add_modifiers:expr,
        $upper_sub_modifiers:expr,
        $lower_fg:expr,
        $lower_bg:expr,
        $lower_add_modifiers:expr,
        $lower_sub_modifiers:expr,
    ) => {
        #[derive(Debug, Default, Clone, clap::Args)]
        #[clap(next_help_heading = $help_heading)]
        pub struct $struct {
            /// Foreground color
            #[clap(name = $upper_fg, long = $lower_fg, arg_enum)]
            fg: Option<$crate::style::Color>,
            /// Background color
            #[clap(name = $upper_bg, long = $lower_bg, arg_enum)]
            bg: Option<$crate::style::Color>,
            /// Add style modifiers
            #[clap(name = $upper_add_modifiers, long = $lower_add_modifiers, parse(try_from_str = modifier_from_str), multiple_values = true, arg_enum)]
            add_modifiers: Vec<$crate::style::Modifier>,
            /// Remove style modifiers
            #[clap(name = $upper_sub_modifiers, long = $lower_sub_modifiers, parse(try_from_str = modifier_from_str), multiple_values = true, arg_enum)]
            sub_modifiers: Vec<$crate::style::Modifier>,
        }

        impl From<$struct> for tui::style::Style {
            fn from(style: $struct) -> Self {
                tui::style::Style {
                    fg: style.fg.map(Into::into),
                    bg: style.bg.map(Into::into),
                    add_modifier: style.add_modifiers.into_iter().sum(),
                    sub_modifier: style.sub_modifiers.into_iter().sum(),
                }
            }
        }
    };
}
