use crate::utils::StringExt;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    symbols,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::debugger::Debugger;

/// Renders the user interface widgets.
pub fn render(debugger: &Debugger, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(frame.area());

    let inspector = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(layout[1]);

    let inspector_border_set = symbols::border::Set {
        top_left: symbols::line::ROUNDED.horizontal_down,
        ..symbols::border::ROUNDED
    };

    let memory_border_set = symbols::border::Set {
        top_right: symbols::line::NORMAL.vertical_left,
        top_left: symbols::line::NORMAL.vertical_right,
        bottom_left: symbols::line::NORMAL.horizontal_up,
        ..symbols::border::ROUNDED
    };

    frame.render_widget(
        Paragraph::new(debugger.text.clone())
            .block(
                Block::bordered()
                    .title("Text")
                    .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
                    .border_type(BorderType::Rounded)
                    .title_alignment(Alignment::Center),
            )
            .style(Style::default())
            .scroll(debugger.scroll_offset),
        layout[0],
    );

    frame.render_widget(
        Paragraph::new(format!(
            "{}\nOutput:\n{:x?}",
            format!(
                "PC: {:#08}\t{:08}\tRel: {:#08x}\t{:08}\n",
                debugger.interpreter.executor.pc,
                debugger.interpreter.executor.pc,
                debugger.interpreter.executor.rel,
                debugger.interpreter.executor.rel,
            )
            .expand_tabs(8),
            debugger.interpreter.output_history(),
        ))
        .wrap(Wrap::default())
        .block(
            Block::bordered()
                .title("Inspector")
                .border_set(inspector_border_set)
                .borders(Borders::TOP | Borders::RIGHT | Borders::LEFT)
                .title_alignment(Alignment::Center),
        )
        .style(Style::default()),
        inspector[0],
    );

    frame.render_widget(
        Paragraph::new(format!(
            "{}",
            debugger
                .interpreter
                .executor
                .memory
                .to_string()
                .expand_tabs(2)
        ))
        .alignment(Alignment::Right)
        .block(
            Block::bordered()
                .title("Memory")
                .border_set(memory_border_set)
                .title_alignment(Alignment::Center),
        )
        .style(Style::default()),
        inspector[1],
    );
}
