mod app;
mod command;
mod editinglist;
mod statefullist;
mod syntax;
mod ui;

use app::App;
use ui::ui;

use crossterm::event::{self, KeyModifiers};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;

use directories::ProjectDirs;
use nix::unistd::Uid;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::process::exit;

use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn check_if_root_user() -> bool {
    Uid::effective().is_root()
}

fn check_if_config_dir_exists(dir: Option<&str>) -> Option<bool> {
    let app = dir.unwrap_or("");
    let proj_dirs = ProjectDirs::from("_", "_", app)?;
    //println!("{:?}", &proj_dirs.config_dir());

    let result = PathBuf::from(&proj_dirs.config_dir()).try_exists();
    //println!("{:?}", result);
    match result {
        Ok(config_dir) => Some(config_dir),
        Err(_) => Some(false),
    }
}

fn check_if_systemd_config_dir_exists() -> Option<bool> {
    check_if_config_dir_exists(Some("create-systemd-service"))
}

fn prerequisites() {
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

    if let Some(does_config_exist) = check_if_config_dir_exists(None) {
        if !does_config_exist {
            eprintln!("~/.config dir does not exist. Reinstall this program. Bye bye.");
            exit(-1);
        }
    }

    if let Some(does_config_exist) = check_if_systemd_config_dir_exists() {
        if !does_config_exist {
            eprintln!(
                "~/.config/create-systemd-service does not exist. Reinstall this program. Bye bye."
            );
            exit(-1);
        }
    }
}

fn start_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.modifiers == KeyModifiers::CONTROL {
                if let KeyCode::Char('x') = key.code {
                    return Ok(());
                }
            }

            if !app.handle_keyboard(key) {
                return Ok(());
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // first things first
    prerequisites();

    //setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = start_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
