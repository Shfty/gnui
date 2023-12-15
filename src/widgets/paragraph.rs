use clap::Args;
use tui::widgets::{Paragraph as TuiParagraph, Wrap as TuiWrap};

use crate::{
    alignment::Alignment,
    threads::main::{FnDraw, Frame, InputBuffer},
    Style,
};

use super::block::Block;

Style!(
    ParagraphStyle,
    "OPTIONS-PARAGRAPH-STYLE",
    "PARAGRAPH_FG",
    "PARAGRAPH_BG",
    "PARAGRAPH_ADD_MODIFIER",
    "PARAGRAPH_SUB_MODIFIER",
    "paragraph-fg",
    "paragraph-bg",
    "paragraph-add-modifier",
    "paragraph-sub-modifier",
);

#[derive(Debug, Clone, Args)]
#[clap(next_help_heading = "OPTIONS-PARAGRAPH")]
pub struct Paragraph {
    /// Text alignment
    #[clap(short, long, arg_enum)]
    alignment: Option<Alignment>,

    #[clap(flatten)]
    wrap: Wrap,

    #[clap(flatten)]
    scroll: Scroll,

    #[clap(flatten)]
    style: ParagraphStyle,

    #[clap(flatten)]
    block: Block,
}

impl Paragraph {
    pub fn draw(self, buf: InputBuffer) -> impl FnDraw {
        let style = self.style.into();
        let block = self.block.try_into();

        move |f: &mut Frame| {
            let rect = f.size();
            let buf = buf.borrow();

            let widget = TuiParagraph::new(buf.as_str()).style(style).scroll((
                self.scroll.y.unwrap_or_default(),
                self.scroll.x.unwrap_or_default(),
            ));

            let widget = if let Ok(wrap) = self.wrap.try_into() {
                widget.wrap(wrap)
            } else {
                widget
            };

            let widget = if let Ok(block) = block.clone() {
                widget.block(block)
            } else {
                widget
            };

            let widget = if let Some(alignment) = self.alignment {
                widget.alignment(alignment.into())
            } else {
                widget
            };

            f.render_widget(widget, rect);
        }
    }
}

#[derive(Debug, Copy, Clone, Args)]
pub struct Wrap {
    /// Enable line wrapping
    #[clap(short, long)]
    wrap: bool,

    /// Trim leading spaces from wrapped lines
    #[clap(short, long)]
    trim: bool,
}

impl TryFrom<Wrap> for TuiWrap {
    type Error = &'static str;

    fn try_from(value: Wrap) -> Result<Self, Self::Error> {
        if value.wrap {
            Ok(TuiWrap { trim: value.trim })
        } else {
            Err("Wrap is not enabled")
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Args)]
struct Scroll {
    /// Horizontal scroll offset
    #[clap(name = "SCROLL_X", short = 'x', long = "scroll-x")]
    x: Option<u16>,
    /// Horizontal scroll offset
    #[clap(name = "SCROLL_Y", short = 'y', long = "scroll-y")]
    y: Option<u16>,
}
