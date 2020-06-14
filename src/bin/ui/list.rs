use tui::widgets::ListState;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

#[cfg(test)]
mod test {
    use crate::ui::list::StatefulList;

    #[test]
    fn stateful_list_test() {
        let items = vec!["one", "two", "three", "four", "five"];

        let mut list = StatefulList::with_items(items);
        assert_eq!(list.state.selected(), None);

        list.next();
        assert_eq!(list.state.selected(), Some(0));

        list.previous();
        assert_eq!(list.state.selected(), Some(4));

        list.next();
        list.next();
        list.next();
        list.next();
        list.next();
        list.next();
        assert_eq!(list.state.selected(), Some(0));

        list.unselect();
        assert_eq!(list.state.selected(), None);
    }
}
