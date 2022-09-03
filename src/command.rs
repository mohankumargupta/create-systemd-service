use std::collections::HashMap;

use crate::app::AppState;

pub struct MenuCommand {
    pub name: &'static str,
    pub shortcut: &'static str,
}

impl MenuCommand {
    pub fn new(name: &'static str, shortcut: &'static str) -> Self {
        Self { name, shortcut }
    }
}

pub struct MenuCommands {
    pub commands: HashMap<AppState, Vec<MenuCommand>>,
}

impl Default for MenuCommands {
    fn default() -> Self {
        let commands = HashMap::from([
            (
                AppState::SelectServiceTemplate,
                vec![
                    MenuCommand::new("Move Up", "↑"),
                    MenuCommand::new("Move Down", "↑"),
                    MenuCommand::new("Select", "Enter"),
                    MenuCommand::new("Exit", "^X"),
                ],
            ),
            (
                AppState::ChooseServiceName,
                vec![
                    MenuCommand::new("Select", "Enter"),
                    MenuCommand::new("Back", "ESC"),
                    MenuCommand::new("Exit", "^X"),
                ],
            ),
            (
                AppState::EnteringEditMode,
                vec![
                    MenuCommand::new("Confirm", "Enter"),
                    MenuCommand::new("Back", "ESC"),
                    MenuCommand::new("Exit", "^X"),
                ],
            ),
            (
                AppState::ViewService,
                vec![
                    MenuCommand::new("Move Up", "↑"),
                    MenuCommand::new("Move Down", "↑"),
                    MenuCommand::new("Edit", "Enter"),
                    MenuCommand::new("Save", "^S"),
                    MenuCommand::new("Exit", "^X"),
                ],
            ),
            (
                AppState::ModifyingService,
                vec![
                    MenuCommand::new("Confirm", "Enter"),
                    MenuCommand::new("Back", "ESC"),
                    MenuCommand::new("Exit", "^X"),
                ],
            ),
        ]);

        Self { commands }
    }
}
