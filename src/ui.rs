use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Tabs},
    Frame,
};

use crate::{
    app::{App, AppState},
    command::MenuCommands,
    syntax::SyntaxText,
};

const TOP_SECTION: usize = 0;
const MAIN_SECTION: usize = 1;
const BOTTOM_SECTION: usize = 2;

const MAIN_LHS: usize = 0;
const MAIN_RHS: usize = 1;

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

    frame.render_widget(tabs, chunks[TOP_SECTION]);

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

            let systemd_detail = List::from(styled_contents);

            let main_section_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(chunks[MAIN_SECTION]);

            frame.render_widget(Clear, main_section_chunks[MAIN_RHS]);
            frame.render_widget(systemd_detail, main_section_chunks[MAIN_RHS]);
            frame.render_stateful_widget(
                list,
                main_section_chunks[MAIN_LHS],
                &mut app.lhs_list.state,
            );

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
        AppState::EnteringEditMode => {
            app.initialise_edit();
        }
        _ => (),
    }

    let commands_map = MenuCommands::default();
    let commands = commands_map.commands.get(&app.app_state).unwrap();

    let mut line_span = Spans(Vec::with_capacity(commands.iter().count()));
    for command in commands.iter() {
        line_span.0.push(Span::styled(
            command.shortcut,
            Style::default()
                .bg(Color::White)
                .fg(Color::Black)
                .patch(Style::default().add_modifier(Modifier::BOLD)),
        ));
        line_span
            .0
            .push(Span::from(" ".to_string() + command.name + "   "));
    }

    let commands_text = Text::from(line_span);
    let commands_paragraph = Paragraph::new(commands_text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Commands")
                .border_type(BorderType::Plain),
        );

    frame.render_widget(commands_paragraph, chunks[BOTTOM_SECTION]);

    match app.app_state {
        AppState::EnteringEditMode | AppState::ViewService | AppState::ModifyingService => {
            let s = &app.selected_template_contents();
            let syntax_text = SyntaxText::new(s);
            let items: Vec<Spans> = syntax_text.into();
            let content_list_items: Vec<ListItem> = items
                .iter()
                .map(|s| ListItem::new(Text::from(s.clone())))
                .collect();

            let systemd_detail = List::new(content_list_items).highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .patch(Style::default().bg(Color::DarkGray))
                    .patch(Style::default().fg(Color::White)),
            );

            //frame.render_widget(Clear, chunks[1]);
            let loo = &mut app.editing_service.state;
            //let mut qoo = loo.clone();
            frame.render_stateful_widget(systemd_detail, chunks[1], loo);
            //frame.render_widget(input2, chunks[1]);

            if app.app_state == AppState::ModifyingService {
                let (key, value) = app.altered_line.as_ref().unwrap();

                let input2 = Paragraph::new(value.to_string())
                    .style(Style::default().fg(Color::White))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Yellow))
                            .title(key.to_string()),
                    );

                let area2 = centered_rect(60, 15, frame.size());
                frame.render_widget(Clear, area2); //this clears out the background
                frame.render_widget(input2, area2);
            }
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
