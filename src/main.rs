use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use directories::ProjectDirs;
use nix::unistd::Uid;
use std::io;
use std::path::PathBuf;
use std::process::exit;
use tui::widgets::List;
use tui::widgets::ListItem;
use tui::widgets::ListState;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Terminal,
};

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Quit,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Quit => 1,
        }
    }
}

pub trait SystemdTemplate {
    fn get_template_from_disk() -> String;
}

struct PremadeSystemdTemplate;

impl SystemdTemplate for PremadeSystemdTemplate {
    fn get_template_from_disk() -> std::string::String {
        if cfg!(debug_assertions) {
            std::fs::read_to_string("data/simple1.service")
                .unwrap_or("Can't open simple1.service ".to_string())
        } else {
            "".to_string()
        }
    }
}

fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "pet-CLI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'p' to access pets, 'a' to add random new pets and 'd' to delete the currently selected pet.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}

fn render_main_contents<'a>(systemd_list_state: &ListState) -> (List<'a>, Paragraph<'a>) {
    let templates_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Select a template")
        .border_type(BorderType::Plain);

    let templates_list = vec!["Simple 1", "Simple 2", "Simple 3"];
    let items: Vec<_> = templates_list
        .iter()
        .map(|template| {
            ListItem::new(Spans::from(vec![Span::styled(
                template.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_template = templates_list
        .get(
            systemd_list_state
                .selected()
                .expect("there is always a selected pet"),
        )
        .expect("exists")
        .clone();
    let list = List::new(items).block(templates_block).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let systemd_detail = Paragraph::new("boo")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Copyright")
                .border_type(BorderType::Plain),
        );

    (list, systemd_detail)
}

fn check_if_root_user() -> bool {
    Uid::effective().is_root()
}

fn check_if_config_dir_exists() -> Option<bool> {
    let proj_dirs = ProjectDirs::from("_", "_", "")?;
    println!("{:?}", &proj_dirs.config_dir());

    let result = PathBuf::from(&proj_dirs.config_dir()).try_exists();
    println!("{:?}", result);
    match result {
        Ok(config_dir) => Some(config_dir),
        Err(_) => Some(false),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !check_if_root_user() {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("WARNING: User does not have root privileges. This will mean you cannot write to /etc/system/system. Start program as root if you want to write to this location. Quit?")
            .default(true)
            .interact()
            .unwrap()
        {
            exit(0);
        }
    }

    if let Some(does_config_exist) = check_if_config_dir_exists() {
        if does_config_exist {
            println!("Dir exists");
        } else {
            println!("Dir does not exist");
        }
    } else {
        println!("Dir really really does not exist");
    }

    exit(0);
    enable_raw_mode().expect("can run in raw mode");
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["Home", "Quit"];
    let mut active_menu_item = MenuItem::Home;

    let simple1 = PremadeSystemdTemplate::get_template_from_disk();

    terminal.draw(|rect| {
        let size = rect.size();
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

        let menu = menu_titles
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
            .select(active_menu_item.into())
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

        rect.render_widget(tabs, chunks[0]);
        //rect.render_widget(render_home(), chunks[1]);
        let pets_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(chunks[1]);

        let mut systemd_list_state = ListState::default();
        systemd_list_state.select(Some(0));
        let (left, right) = render_main_contents(&systemd_list_state);
        rect.render_stateful_widget(left, pets_chunks[0], &mut systemd_list_state);
        rect.render_widget(right, pets_chunks[1]);
        rect.render_widget(copyright, chunks[2]);
    })?;
    disable_raw_mode().expect("should work");
    Ok(())
}
