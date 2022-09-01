use regex::Regex;
use tui::widgets::ListState;

/// List widget with TUI controlled states.
#[derive(Debug)]
pub struct EditingList {
    /// List items (states).
    //pub items: Vec<Spans<'a>>,
    /// State that can be modified by TUI.
    pub state: ListState,
    pub editing_text: Vec<String>,
}

impl Default for EditingList {
    fn default() -> Self {
        let mut result = Self {
            state: ListState::default(),
            editing_text: vec![],
        };
        result.state.select(Some(0));
        result
    }
}

impl EditingList {
    pub fn next(&mut self) {
        let i = self.state.selected().unwrap();
        let len = self.editing_text.len();

        if i > len {
            return;
        }

        let re: Regex = Regex::new(r"^(?P<key>[^=;#]+)=(?P<value>[^;#]*)").unwrap();
        let range = (i + 1)..len;
        for x in range {
            if re.is_match(&self.editing_text[x]) {
                self.state.select(Some(x));
                return;
            }
        }
    }

    pub fn previous(&mut self) {
        let i = self.state.selected().unwrap();

        if i <= 0 {
            return;
        }

        let re: Regex = Regex::new(r"^(?P<key>[^=;#]+)=(?P<value>[^;#]*)").unwrap();
        let range = 0..=(i - 1);
        for x in range.rev() {
            if re.is_match(&self.editing_text[x]) {
                self.state.select(Some(x));
                return;
            }
        }
    }

    /*
    /// Selects the next item.
    pub fn next2(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.item_count - 1 {
                    self.item_count - 1
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    */

    /*
    /// Selects the previous item.
    pub fn previous2(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    */

    pub fn first(&mut self) {
        self.state.select(Some(0));
        self.next();
    }

    pub fn last(&mut self) {
        self.state.select(Some(self.editing_text.len() - 1));
        self.previous();
    }

    pub fn get_selected_key_value(&mut self) -> (&str, &str) {
        let index = self.state.selected().unwrap();
        let selected_line = self.editing_text[index].as_str();
        let mut key_value = selected_line.split("=");
        let key = key_value.next().unwrap();
        let value = key_value.next().unwrap();
        (key, value)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stateful_list() {
        let mut list = StatefulList::with_items(vec!["data1", "data2", "data3"]);
        list.state.select(Some(1));
        assert_eq!(Some(&"data2"), list.selected());
        list.next();
        assert_eq!(Some(2), list.state.selected());
        list.previous();
        assert_eq!(Some(1), list.state.selected());
    }
}
*/
