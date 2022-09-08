use std::{
    fs::{self, File},
    io::{BufWriter, Write},
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use directories::ProjectDirs;
use nix::unistd::Uid;
use regex::Regex;
use tui::widgets::ListState;

use crate::{editinglist::EditingList, statefullist::StatefulList};

#[derive(PartialEq, Eq, Hash)]
pub enum AppState {
    SelectServiceTemplate,
    ChooseServiceName,
    EnteringEditMode,
    ViewService,
    ModifyingService,
}

pub struct App {
    pub lhs_list: StatefulList<String>,
    pub app_state: AppState,
    pub service_name: String,
    pub editing_service: EditingList,
    pub altered_line: Option<(String, String)>,
    pub just_saved: bool,
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
            editing_service: EditingList::default(),
            altered_line: None,
            just_saved: false,
        };
        app.lhs_list.state.select(Some(0));
        //app.rhs_list_state.select(Some(0));
        app
    }

    pub fn handle_keyboard(&mut self, key: KeyEvent) -> bool {
        if key.modifiers == KeyModifiers::CONTROL {
            if let KeyCode::Char(c) = key.code {
                if c == 'v' {
                    return false;
                }

                if c == 's' {
                    if self.app_state == AppState::ViewService {
                        self.save();
                        self.just_saved = true;
                    }
                };
            }
            return true;
        }

        let returncode = true;

        match key.code {
            KeyCode::Enter => match self.app_state {
                AppState::SelectServiceTemplate => self.app_state = AppState::ChooseServiceName,
                AppState::ChooseServiceName => self.app_state = AppState::EnteringEditMode,
                AppState::ViewService => {
                    self.store_key_value();
                }
                AppState::EnteringEditMode => (),
                AppState::ModifyingService => {
                    self.modify_unit();
                    self.app_state = AppState::ViewService;
                }
            },
            KeyCode::Left => (),
            KeyCode::Right => (),
            KeyCode::Up => match self.app_state {
                AppState::SelectServiceTemplate => self.lhs_list.previous(),
                AppState::ChooseServiceName => (),
                AppState::ViewService => self.previous_content_item(),
                AppState::EnteringEditMode => (),
                AppState::ModifyingService => (),
            },
            KeyCode::Down => match self.app_state {
                AppState::SelectServiceTemplate => self.lhs_list.next(),
                AppState::ChooseServiceName => (),
                AppState::ViewService => self.next_content_item(),
                AppState::EnteringEditMode => (),
                AppState::ModifyingService => (),
            },
            KeyCode::PageUp => self.first_content_item(),
            KeyCode::PageDown => self.last_content_item(),
            KeyCode::Delete => (),
            KeyCode::F(_) => (),
            KeyCode::Char(ch) => match self.app_state {
                AppState::SelectServiceTemplate => {
                    if ch == 'q' {
                        return false;
                    }
                }
                AppState::ChooseServiceName => self.service_name.push(ch),
                AppState::ViewService => (),
                AppState::EnteringEditMode => (),
                AppState::ModifyingService => {
                    self.modifying_service_push(ch);
                }
            },
            KeyCode::Backspace => {
                if let AppState::ChooseServiceName = self.app_state {
                    self.service_name.pop();
                } else if let AppState::ModifyingService = self.app_state {
                    self.modifying_service_pop();
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
        let index = self
            .lhs_list
            .state
            .selected()
            .expect("Highlighted value should always be valid.");
        let editing_text = self
            .lhs_list
            .items
            .get(index)
            .expect("Highlighted value should always be valid.")
            .1
            .to_string();
        self.editing_service.editing_text = editing_text.lines().map(|s| s.to_owned()).collect();
        self.editing_service.next();

        self.app_state = AppState::ViewService;
    }

    fn next_content_item(&mut self) {
        self.editing_service.next();
    }

    fn previous_content_item(&mut self) {
        self.editing_service.previous();
    }

    fn first_content_item(&mut self) {
        self.editing_service.first();
    }

    fn last_content_item(&mut self) {
        self.editing_service.last();
    }

    pub fn selected_template_contents(&self) -> String {
        self.editing_service.editing_text.join("\n")
    }

    fn store_key_value(&mut self) {
        let (key, value) = self.editing_service.get_selected_key_value();
        self.altered_line = Some((key.to_string(), value.to_string()));
        self.app_state = AppState::ModifyingService;
    }

    fn modifying_service_push(&mut self, ch: char) {
        let s = self.altered_line.as_ref().unwrap().1.as_str();
        let t = self.altered_line.as_ref().unwrap().0.as_str();
        let new_s = s.to_string() + &ch.to_string();
        self.altered_line = Some((t.to_string(), new_s));
    }

    fn modifying_service_pop(&mut self) {
        let t = self.altered_line.as_ref().unwrap().0.as_str();
        let mut u = self.altered_line.as_ref().unwrap().1.to_string();
        u.pop();
        self.altered_line = Some((t.to_string(), u));
    }

    fn modify_unit(&mut self) {
        //let (key, _value) = self.altered_line.as_ref().unwrap();
        let index = self.editing_service.state.selected().unwrap();
        let item = &self.editing_service.editing_text[index];
        let new_text = &self.altered_line.as_ref().unwrap().1.to_string();
        let re = Regex::new(r"^(?P<key>[^=;#]+)=(?P<value>[^;#]*)").unwrap();
        let new_line = re.replace_all(item, format!("$key={new_text}")).to_string();
        self.editing_service.editing_text[index] = new_line;
    }

    fn save(&mut self) {
        let is_root = Uid::effective().is_root();
        let file_name: String;
        let service_name = self.service_name.to_string();
        if is_root {
            file_name = format!("/etc/systemd/system/{service_name}.service");
        } else {
            file_name = format!("~/.config/systemd/user/{service_name}.service")
        }

        let f = File::create(file_name).expect("Unable to create file");
        let mut f = BufWriter::new(f);
        let result = self
            .editing_service
            .editing_text
            .iter()
            .fold(String::new(), |acc, l| acc + l + "\n");
        f.write_all(result.as_bytes()).expect("all good");
    }
}
