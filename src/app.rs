use std::fs;

use directories::ProjectDirs;
use tui::widgets::ListState;

use crate::statefullist::StatefulList;

pub struct App {
    pub lhs_list: StatefulList<String>,
    pub template_names: Vec<String>,
    pub template_contents: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        let (template_names, template_contents) = Self::find_service_templates();
        let mut lhs_list_state = ListState::default();
        lhs_list_state.select(Some(0));
        let mut app = App {
            lhs_list: StatefulList::with_items(template_names.clone()),
            template_names: template_names.clone(),
            template_contents: template_contents,
        };
        app.lhs_list.state.select(Some(0));
        app
    }

    pub fn find_service_templates() -> (Vec<String>, Vec<String>) {
        let proj_dirs = ProjectDirs::from("_", "_", "create-systemd-service").unwrap();
        let templates_path = proj_dirs.config_dir();

        let mut v1: Vec<String> = vec![];
        let mut v2: Vec<String> = vec![];

        for template_result in fs::read_dir(templates_path).unwrap() {
            if let Ok(template) = template_result {
                let file_name = template.file_name().into_string().unwrap();
                if file_name.contains(".service") {
                    let template_name = file_name.replace(".service", "");
                    let template_contents = fs::read_to_string(template.path()).unwrap();
                    v1.push(template_name);
                    v2.push(template_contents);
                }
            }
        }

        (v1, v2)
    }
}
