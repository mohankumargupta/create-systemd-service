use std::fs;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use directories::ProjectDirs;
use tui::widgets::ListState;

use crate::statefullist::StatefulList;

#[derive(PartialEq)]
pub enum AppState {
    SelectServiceTemplate,
    ChooseServiceName,
}

pub struct App {
    pub lhs_list: StatefulList<String>,
    pub app_state: AppState,
    pub service_name: String,
}

impl App {
    pub fn new() -> Self {
        let templates = Self::find_service_templates();
        let mut lhs_list_state = ListState::default();
        lhs_list_state.select(Some(0));

        let mut app = App {
            lhs_list: StatefulList::with_items(templates.clone()),
            app_state: AppState::SelectServiceTemplate,
            service_name: "".to_string(),
        };
        app.lhs_list.state.select(Some(0));
        app
    }

    pub fn handle_keyboard(&mut self, key: KeyEvent) {
        if key.modifiers == KeyModifiers::CONTROL {
            if let KeyCode::Char(c) = key.code {
                match c {
                    'v' => (),
                    _ => (),
                }
                return;
            }
        }

        match key.code {
            KeyCode::Enter => match self.app_state {
                AppState::SelectServiceTemplate => self.app_state = AppState::ChooseServiceName,
                AppState::ChooseServiceName => (),
            },
            KeyCode::Left => (),
            KeyCode::Right => (),
            KeyCode::Up => self.lhs_list.previous(),
            KeyCode::Down => self.lhs_list.next(),
            KeyCode::Delete => (),
            KeyCode::F(_) => (),
            KeyCode::Char(ch) => match self.app_state {
                AppState::SelectServiceTemplate => (),
                AppState::ChooseServiceName => self.service_name.push(ch),
            },
            KeyCode::Backspace => {
                if let AppState::ChooseServiceName = self.app_state {
                    self.service_name.pop();
                }
            }
            KeyCode::Esc => {
                if let AppState::ChooseServiceName = self.app_state {
                    self.app_state = AppState::SelectServiceTemplate;
                    self.service_name.clear();
                }
            }
            KeyCode::Modifier(_) => (),
            _ => (),
        }
    }

    pub fn find_service_templates() -> Vec<(String, String)> {
        let proj_dirs = ProjectDirs::from("_", "_", "create-systemd-service").unwrap();
        let templates_path = proj_dirs.config_dir();

        let mut v1: Vec<(String, String)> = vec![];

        for template_result in fs::read_dir(templates_path).unwrap() {
            if let Ok(template) = template_result {
                let file_name = template.file_name().into_string().unwrap();
                if file_name.contains(".service") {
                    let template_name = file_name.replace(".service", "");
                    let template_contents = fs::read_to_string(template.path()).unwrap();
                    v1.push((template_name, template_contents));
                }
            }
        }
        v1.sort_by_key(|f| f.0.clone());
        v1
    }
}
