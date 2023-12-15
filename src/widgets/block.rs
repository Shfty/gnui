use crate::{
    alignment::Alignment,
    border::{Border, BorderType},
    Style,
};
use clap::Args;
use tui::widgets::Block as TuiBlock;

Style!(
    BlockStyle,
    "OPTIONS-BLOCK-STYLE",
    "BLOCK_FG",
    "BLOCK_BG",
    "BLOCK_ADD_MODIFIER",
    "BLOCK_SUB_MODIFIER",
    "block-fg",
    "block-bg",
    "block-add-modifier",
    "block-sub-modifier",
);

Style!(
    BlockBorderStyle,
    "OPTIONS-BLOCK-BORDER-STYLE",
    "BLOCK_BORDER_FG",
    "BLOCK_BORDER_BG",
    "BLOCK_BORDER_ADD_MODIFIER",
    "BLOCK_BORDER_SUB_MODIFIER",
    "block-border-fg",
    "block-border-bg",
    "block-border-add-modifier",
    "block-border-sub-modifier",
);

#[derive(Debug, Default, Clone, Args)]
#[clap(next_help_heading = "OPTIONS-BLOCK")]
pub struct Block {
    /// Draw inside a block widget
    #[clap(name = "BLOCK", long = "block")]
    pub enabled: bool,

    /// Title displayed at the top of a block widget
    #[clap(name = "BLOCK_TITLE", long = "block-title")]
    pub title: Option<String>,
    #[clap(
        name = "BLOCK_TITLE_ALIGNMENT",
        long = "block-title-alignment",
        arg_enum
    )]
    pub title_alignment: Option<Alignment>,

    #[clap(flatten)]
    pub style: BlockStyle,

    #[clap(flatten)]
    pub border: BlockBorder,
}

impl TryFrom<Block> for TuiBlock<'_> {
    type Error = ();

    fn try_from(block: Block) -> std::result::Result<Self, Self::Error> {
        if !block.enabled {
            return Err(());
        }

        let out = TuiBlock::default()
            .borders(block.border.borders.into_iter().sum())
            .border_style(block.border.style.into())
            .style(block.style.into());

        let out = if let Some(title) = block.title {
            out.title(title)
        } else {
            out
        };

        let out = if let Some(title_alignment) = block.title_alignment {
            out.title_alignment(title_alignment.into())
        } else {
            out
        };

        let out = if let Some(border_type) = block.border.ty {
            out.border_type(border_type.into())
        } else {
            out
        };

        Ok(out)
    }
}

#[derive(Debug, Default, Clone, Args)]
#[clap(next_help_heading = "OPTIONS-BLOCK-BORDER")]
pub struct BlockBorder {
    /// Draw borders on the specified sides
    #[clap(name = "BLOCK_BORDER", long = "block-border", arg_enum)]
    pub borders: Vec<Border>,

    /// The character used to render block borders
    #[clap(name = "BLOCK_BORDER_TYPE", long = "block-border-type", arg_enum)]
    pub ty: Option<BorderType>,

    #[clap(flatten)]
    pub style: BlockBorderStyle,
}
