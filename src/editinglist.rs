use tui::widgets::ListState;

/// List widget with TUI controlled states.
#[derive(Debug)]
pub struct EditingList {
    /// List items (states).
    //pub items: Vec<Spans<'a>>,
    /// State that can be modified by TUI.
    pub state: ListState,
    pub item_count: usize,
    pub editing_text: Vec<String>,
}

impl Default for EditingList {
    fn default() -> Self {
        let mut result = Self {
            state: ListState::default(),
            item_count: 0,
            editing_text: vec![],
        };
        result.state.select(Some(0));
        result
    }
}

impl EditingList {
    /// Constructs a new instance of `StatefulList`.
    /*
        pub fn new(mut state: ListState, item_count: usize) -> EditingList {
            state.select(Some(0));
            //state.select(None);
            Self { state, item_count }
        }
    */
    /// Construct a new `StatefulList` with given items.
    /*
    pub fn with_items(items: Vec<Spans>) -> EditingList {
        Self::new(items, ListState::default())
    }
    */

    /// Returns the selected item.

    /*
    fn selected(&self) -> Option<&Spans<'a>> {
        self.items.get(self.state.selected()?)
    }
    */

    /// Selects the next item.
    pub fn next(&mut self) {
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

    /// Selects the previous item.
    pub fn previous(&mut self) {
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

    pub fn first(&mut self) {
        self.state.select(Some(0));
    }

    pub fn last(&mut self) {
        self.state.select(Some(self.item_count - 1));
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
