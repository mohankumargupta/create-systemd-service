use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Tabs},
    Frame,
};

use crate::{
    app::{App, AppState},
    syntax::SyntaxText,
};

pub fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let size = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size);

    let menu_titles = vec!["Home", "Quit"];
    let menu: Vec<Spans> = menu_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    let tabs = Tabs::new(menu)
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(0)
        .divider(Span::raw("|"));

    let copyright = Paragraph::new("pet-CLI 2020 - all rights reserved")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Copyright")
                .border_type(BorderType::Plain),
        );

    frame.render_widget(tabs, chunks[0]);
    frame.render_widget(copyright, chunks[2]);

    match app.app_state {
        AppState::SelectServiceTemplate | AppState::ChooseServiceName => {
            let templates_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Select a template")
                .border_type(BorderType::Plain);

            let items: Vec<_> = app
                .lhs_list
                .items
                .iter()
                .map(|(template, _)| {
                    ListItem::new(Spans::from(vec![Span::styled(
                        template.clone(),
                        Style::default(),
                    )]))
                })
                .collect();

            let list = List::new(items).block(templates_block).highlight_style(
                Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            );

            let &(_, template_contents) = &app
                .lhs_list
                .items
                .get(app.lhs_list.state.selected().unwrap())
                .unwrap();

            let styled_contents = SyntaxText::new(&template_contents);

            let systemd_detail = List::from(styled_contents)
                .highlight_style(Style::default().bg(Color::Rgb(117, 113, 94)));

            let pets_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(chunks[1]);

            frame.render_widget(Clear, pets_chunks[1]);
            frame.render_stateful_widget(systemd_detail, pets_chunks[1], &mut app.rhs_list_state);
            frame.render_stateful_widget(list, pets_chunks[0], &mut app.lhs_list.state);

            let input = Paragraph::new(app.service_name.as_ref())
                .style(Style::default().fg(Color::White))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Yellow))
                        .title("Name of Service"),
                );

            if app.app_state == AppState::ChooseServiceName {
                let area = centered_rect(60, 15, frame.size());
                frame.render_widget(Clear, area); //this clears out the background
                frame.render_widget(input, area);
            }
        }
        AppState::EditService => (),
        AppState::EnteringEditMode => app.initialise_edit(),
    }

    match app.app_state {
        AppState::EnteringEditMode | AppState::EditService => {
            let input2 = Paragraph::new(app.editing_text.as_ref())
                .style(Style::default().fg(Color::White))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Yellow))
                        .title("Name of Service"),
                );
            frame.render_widget(Clear, chunks[1]);
            frame.render_widget(input2, chunks[1]);
        }
        _ => (),
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
