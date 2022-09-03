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
                AppState::ChooseServiceName,
                vec![
                    MenuCommand::new("↑", "Move Up"),
                    MenuCommand::new("↑", "Move Down"),
                    MenuCommand::new("Enter", "Select"),
                    MenuCommand::new("^X", "Exit"),
                ],
            ),
            (
                AppState::EnteringEditMode,
                vec![
                    MenuCommand::new("Enter", "Confirm"),
                    MenuCommand::new("ESC", "Back"),
                    MenuCommand::new("^X", "Exit"),
                ],
            ),
            (
                AppState::ViewService,
                vec![
                    MenuCommand::new("↑", "Move Up"),
                    MenuCommand::new("↑", "Move Down"),
                    MenuCommand::new("Enter", "Edit"),
                    MenuCommand::new("^S", "Save"),
                    MenuCommand::new("^X", "Exit"),
                ],
            ),
            (
                AppState::ModifyingService,
                vec![
                    MenuCommand::new("Enter", "Confirm"),
                    MenuCommand::new("ESC", "Back"),
                    MenuCommand::new("^X", "Exit"),
                ],
            ),
        ]);

        Self { commands }
    }
}
