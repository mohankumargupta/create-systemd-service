use std::fs;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use directories::ProjectDirs;
use tui::widgets::ListState;

use crate::{editinglist::EditingList, statefullist::StatefulList};

#[derive(PartialEq)]
pub enum AppState {
    SelectServiceTemplate,
    ChooseServiceName,
    EnteringEditMode,
    EditService,
}

pub struct App<'a> {
    pub lhs_list: StatefulList<String>,
    pub rhs_list_state: ListState,
    pub app_state: AppState,
    pub service_name: String,
    pub editing_service: Option<EditingList<'a>>,
    pub editing_text: String,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let templates = Self::find_service_templates();
        let mut lhs_list_state = ListState::default();
        lhs_list_state.select(Some(0));

        let mut app = App {
            lhs_list: StatefulList::with_items(templates.clone()),
            rhs_list_state: ListState::default(),
            app_state: AppState::SelectServiceTemplate,
            service_name: "".to_string(),
            editing_service: None,
            editing_text: "".to_string(),
        };
        app.lhs_list.state.select(Some(0));
        //app.rhs_list_state.select(Some(0));
        app
    }

    pub fn handle_keyboard(&mut self, key: KeyEvent) -> bool {
        if key.modifiers == KeyModifiers::CONTROL {
            if let KeyCode::Char(c) = key.code {
                match c {
                    'v' => return false,
                    _ => (),
                }
            }
        }

        let returncode = true;

        match key.code {
            KeyCode::Enter => match self.app_state {
                AppState::SelectServiceTemplate => self.app_state = AppState::ChooseServiceName,
                AppState::ChooseServiceName => {
                    self.app_state = AppState::EnteringEditMode;
                    //self.initialise_edit();
                }
                AppState::EditService => (),
                AppState::EnteringEditMode => (),
            },
            KeyCode::Left => (),
            KeyCode::Right => (),
            KeyCode::Up => match self.app_state {
                AppState::SelectServiceTemplate => self.lhs_list.previous(),
                AppState::ChooseServiceName => (),
                AppState::EditService => (),
                AppState::EnteringEditMode => (),
            },
            KeyCode::Down => match self.app_state {
                AppState::SelectServiceTemplate => self.lhs_list.next(),
                AppState::ChooseServiceName => (),
                AppState::EditService => (),
                AppState::EnteringEditMode => (),
            },
            KeyCode::Delete => (),
            KeyCode::F(_) => (),
            KeyCode::Char(ch) => match self.app_state {
                AppState::SelectServiceTemplate => {
                    if ch == 'q' {
                        return false;
                    }
                }
                AppState::ChooseServiceName => self.service_name.push(ch),
                AppState::EditService => (),
                AppState::EnteringEditMode => (),
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

        returncode
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

    pub fn initialise_edit(&mut self) {
        let index = self.lhs_list.state.selected().unwrap();
        self.editing_text = self.lhs_list.items.get(index).unwrap().1.to_string();

        self.editing_service = Some(EditingList::with_items(vec![]));

        self.app_state = AppState::EditService;
    }

    /*
    pub fn setEditingText(&mut self, content: String) {
        self.editing_text = content;
    }
    */
}
