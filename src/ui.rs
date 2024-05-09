use crate::app::{CurrentScreen, CurrentlyInputting};
use ratatui::Frame;
use ratatui::prelude::Layout;

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

pub fn ui(f: &mut Frame, app: &crate::app::App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());
    
    // Render the title
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Create new Graph",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    // bottom navigation bar :: begin
    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => {
                Span::styled("Normal Mode", Style::default().fg(Color::LightGreen))
            }
            CurrentScreen::Success => {
                Span::styled("Display Mode", Style::default().fg(Color::Green))
            }
            CurrentScreen::Failure => {
                Span::styled("Failure Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => {
                Span::styled("Exiting", Style::default().fg(Color::LightRed))
            }
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            if let Some(inputting) = &app.currently_inputting {
                match inputting {
                    CurrentlyInputting::Expression => Span::styled(
                        "Inputting Expression",
                        Style::default().fg(Color::Green),
                    ),
                    CurrentlyInputting::Xdomain => Span::styled(
                        "Inputting X-Domain",
                        Style::default().fg(Color::LightGreen),
                    ),
                }
            } else {
                Span::styled(
                    "Not Inputting Anything",
                    Style::default().fg(Color::DarkGray),
                )
            }
        },
    ];

    // hint in navigation bar for available keys :: begin
    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));
    // bottom naviagtion bar ::end

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(Tab) to change input field, (Enter) to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Success => Span::styled(
                "(q) to quit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Failure => Span::styled(
                "(q) to quit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default().borders(Borders::ALL));
    // hint in navigation bar for available keys :: end

    // new layout in this space :: begin
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);
    // new layout in this space :: end

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);

    if let CurrentScreen::Success = app.current_screen {
        f.render_widget(Clear, f.size());
        let dataset = vec![
            Dataset::default()
                .name("Graph")
                .marker(Marker::Dot)
                .graph_type(GraphType::Scatter)
                .style(Style::default().cyan())
                .data(&app.graph_vector)
        ];

        let x_min_clone = app.x_min.clone();
        let x_max_clone = app.x_max.clone();
        let x_axis = Axis::default()
            .title("X Axis".red())
            .style(Style::default().white())
            .bounds([x_min_clone, x_max_clone])
            .labels(vec![x_min_clone.to_string().into(), x_max_clone.to_string().into()]);

        let y_min_clone = app.y_min.clone();
        let y_max_clone = app.y_max.clone();
        let y_axis = Axis::default()
            .title("Y Axis".red())
            .style(Style::default().white())
            .bounds([y_min_clone, y_max_clone])
            .labels(vec![y_min_clone.to_string().into(), y_max_clone.to_string().into()]);

        let chart = Chart::new(dataset)
            .block(Block::default().title("Graph"))
            .x_axis(x_axis)
            .y_axis(y_axis);
        let area = centered_rect(50, 50, f.size());
        f.render_widget(chart, area)
    }

    if let CurrentScreen::Exiting = app.current_screen {
        f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to quit?",
            Style::default().fg(Color::Red),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, f.size());
        f.render_widget(exit_paragraph, area);
    }

}


// from: https://ratatui.rs/tutorials/json-editor/ui/#:~:text=///%20helper%20function%20to,%7D
/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}