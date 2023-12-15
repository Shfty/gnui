use std::{collections::BTreeMap, num::ParseFloatError};

use crate::{
    style::Color,
    threads::main::{FnDraw, Frame, InputBuffer},
    Style,
};

use clap::Args;

use tui::{
    layout::Constraint,
    style::{Color as TuiColor, Style as TuiStyle},
    symbols::Marker,
    widgets::{Axis as TuiAxis, Chart as TuiChart, Dataset},
};

use super::block::Block;

Style!(
    ChartStyle,
    "OPTIONS-CHART-STYLE",
    "CHART_FG",
    "CHART_BG",
    "CHART_ADD_MODIFIER",
    "CHART_SUB_MODIFIER",
    "chart-fg",
    "chart-bg",
    "chart-add-modifier",
    "chart-sub-modifier",
);

#[derive(Debug, Clone, Args)]
#[clap(next_help_heading = "OPTIONS-CHART")]
pub struct Chart {
    /// Dataset colors, assigned sequentially. Wraps if dataset count exceeds color count.
    #[clap(
        name = "COLOR",
        short = 'c',
        long = "color",
        multiple_values = true,
        arg_enum
    )]
    colors: Vec<Color>,

    #[clap(flatten)]
    style: ChartStyle,

    #[clap(flatten)]
    block: Block,

    #[clap(flatten)]
    x_axis: XAxis,

    #[clap(flatten)]
    y_axis: YAxis,
}

fn bounds_from_str(bounds: &str) -> Result<[f64; 2], ParseFloatError> {
    let mut bounds = bounds.split("..");
    Ok([
        bounds.next().unwrap_or_default().parse()?,
        bounds.next().unwrap_or_default().parse()?,
    ])
}

#[derive(Debug, Clone, Args)]
#[clap(next_help_heading = "OPTIONS-CHART-X-AXIS")]
pub struct XAxis {
    /// X axis label
    #[clap(name = "X_AXIS_TITLE", long = "x-axis-title")]
    title: Option<String>,

    /// X axis value range
    #[clap(name = "X_AXIS_BOUNDS", long = "x-axis-bounds", parse(try_from_str = bounds_from_str), default_value = "0.0..1.0")]
    bounds: [f64; 2],

    #[clap(flatten)]
    style: XAxisStyle,
}

#[derive(Debug, Clone, Args)]
#[clap(next_help_heading = "OPTIONS-CHART-Y-AXIS")]
pub struct YAxis {
    /// Y axis label
    #[clap(name = "Y_AXIS_TITLE", long = "y-axis-title")]
    title: Option<String>,

    /// Y axis value range
    #[clap(name = "Y_AXIS_BOUNDS", long = "y-axis-bounds", parse(try_from_str = bounds_from_str), default_value = "0.0..1.0")]
    bounds: [f64; 2],

    #[clap(flatten)]
    style: YAxisStyle,
}

Style!(
    XAxisStyle,
    "OPTIONS-CHART-X-AXIS-STYLE",
    "X_AXIS_FG",
    "X_AXIS_BG",
    "X_AXIS_ADD_MODIFIER",
    "X_AXIS_SUB_MODIFIER",
    "x-axis-fg",
    "x-axis-bg",
    "x-axis-add-modifier",
    "x-axis-sub-modifier",
);

Style!(
    YAxisStyle,
    "OPTIONS-CHART-Y-AXIS-STYLE",
    "Y_AXIS_FG",
    "Y_AXIS_BG",
    "Y_AXIS_ADD_MODIFIER",
    "Y_AXIS_SUB_MODIFIER",
    "y-axis-fg",
    "y-axis-bg",
    "y-axis-add-modifier",
    "y-axis-sub-modifier",
);

impl Chart {
    pub fn draw(self, buf: InputBuffer) -> impl FnDraw {
        let mut names: BTreeMap<usize, String> = Default::default();
        let mut data: BTreeMap<usize, Vec<(f64, f64)>> = Default::default();

        let style = self.style.into();
        let block = self.block.try_into();
        let x_axis_style = self.x_axis.style.into();
        let y_axis_style = self.y_axis.style.into();
        move |f: &mut Frame| {
            let buf = buf.borrow();
            for (i, line) in buf.lines().enumerate() {
                let mut parts = line.split('\t');

                if let Ok(f) = parts.next().unwrap().parse::<f64>() {
                    names.insert(i, parts.collect::<String>());

                    let entry = data.entry(i).or_insert(vec![(0.0, 0.0); self.x_axis.bounds[1] as usize]);
                    entry.push((entry.len() as f64, f));
                    if entry.len() > 100 {
                        entry.remove(0);
                        for (i, data) in entry.iter_mut().enumerate() {
                            data.0 = i as f64;
                        }
                    }
                }
            }

            let rect = f.size();
            let datasets = data
                .values()
                .enumerate()
                .map(|(i, data)| {
                    let color = if self.colors.len() > 0 {
                        self.colors[i % self.colors.len()].into()
                    } else {
                        TuiColor::White
                    };

                    Dataset::default()
                        .name(&names[&i])
                        .marker(Marker::Braille)
                        .graph_type(tui::widgets::GraphType::Line)
                        .style(TuiStyle::default().fg(color))
                        .data(&data)
                })
                .collect::<Vec<_>>();

            let x_axis = TuiAxis::default()
                .bounds(self.x_axis.bounds)
                .style(x_axis_style);
            let x_axis = if let Some(title) = self.x_axis.title.as_ref().map(String::as_str) {
                x_axis.title(title)
            } else {
                x_axis
            };

            let y_axis = TuiAxis::default()
                .bounds(self.y_axis.bounds)
                .style(y_axis_style);
            let y_axis = if let Some(title) = self.y_axis.title.as_ref().map(String::as_str) {
                y_axis.title(title)
            } else {
                y_axis
            };

            let widget = TuiChart::new(datasets)
                .hidden_legend_constraints((Constraint::Ratio(1, 1), Constraint::Ratio(1, 1)))
                .x_axis(x_axis)
                .y_axis(y_axis)
                .style(style);

            let widget = if let Ok(block) = block.clone() {
                widget.block(block)
            } else {
                widget
            };

            f.render_widget(widget, rect);
        }
    }
}
