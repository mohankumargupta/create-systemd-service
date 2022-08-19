use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Tabs},
    Frame,
};

use crate::app::App;

pub fn ui<B: Backend>(frame: &mut Frame<B>, app: &App) {
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

    let templates_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Select a template")
        .border_type(BorderType::Plain);

    let items: Vec<_> = app
        .template_names
        .iter()
        .map(|template| {
            ListItem::new(Spans::from(vec![Span::styled(
                template.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let mut systemd_list_state = ListState::default();
    systemd_list_state.select(Some(0));

    /*
    let selected_template = app
        .template_names
        .get(
            systemd_list_state
                .selected()
                .expect("there is always a selected pet"),
        )
        .expect("exists")
        .clone();
    */

    let list = List::new(items).block(templates_block).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let systemd_detail = Paragraph::new(&*app.template_contents[0])
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Copyright")
                .border_type(BorderType::Plain),
        );

    let pets_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[1]);

    let mut systemd_list_state = ListState::default();
    frame.render_stateful_widget(list, pets_chunks[0], &mut systemd_list_state);
    frame.render_widget(systemd_detail, pets_chunks[1]);
}
