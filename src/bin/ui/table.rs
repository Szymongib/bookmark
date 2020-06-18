use tui::widgets::TableState;

pub struct StatefulTable<T> {
    pub items: Vec<T>,
    pub state: TableState,
}

impl<T> StatefulTable<T> {
    pub fn new() -> StatefulTable<T> {
        let vec: Vec<T> = vec![];

        StatefulTable {
            state: TableState::default(),
            items: vec,
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulTable<T> {
        StatefulTable {
            state: TableState::default(),
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
    use crate::ui::table::StatefulTable;

    #[test]
    fn stateful_list_test() {
        let items = vec!["one", "two", "three", "four", "five"];

        let mut table = StatefulTable::with_items(items);
        assert_eq!(table.state.selected(), None);

        table.next();
        assert_eq!(table.state.selected(), Some(0));

        table.previous();
        assert_eq!(table.state.selected(), Some(4));

        table.next();
        table.next();
        table.next();
        table.next();
        table.next();
        table.next();
        assert_eq!(table.state.selected(), Some(0));

        table.unselect();
        assert_eq!(table.state.selected(), None);
    }
}
