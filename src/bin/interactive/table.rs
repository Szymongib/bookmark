use tui::widgets::TableState;

pub trait TableItem {
    fn visible(&self) -> bool;
    fn row(&self) -> &Vec<String>;
    fn id(&self) -> String;
}

pub struct StatefulTable<T: TableItem> {
    pub items: Vec<T>,
    pub visible: Vec<T>,
    pub state: TableState,
}

impl<T: TableItem + Clone> StatefulTable<T> {
    pub fn with_items(items: &[T]) -> StatefulTable<T> {
        StatefulTable {
            state: TableState::default(),
            items: items.to_vec(),
            visible: items.to_vec(),
        }
    }

    pub fn refresh_visible(&mut self) {
        self.visible.clear();

        for i in &self.items {
            if i.visible() {
                self.visible.push(i.clone())
            }
        }
    }

    pub fn override_items(&mut self, items: &[T]) {
        self.items = items.to_vec();
        self.visible = items.to_vec();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.visible.len() - 1 {
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
                    self.visible.len() - 1
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
    use crate::interactive::table::StatefulTable;
    use crate::interactive::url_table_item::URLItem;
    use bookmark_lib::record_filter::URLFilter;
    use bookmark_lib::types::URLRecord;

    fn fix_url_items() -> Vec<URLItem> {
        vec![
            URLItem::new(URLRecord::new("one", "one", "one", vec![])),
            URLItem::new(URLRecord::new("two", "two", "two", vec![])),
            URLItem::new(URLRecord::new("three", "three", "three", vec![])),
            URLItem::new(URLRecord::new("four", "four", "four", vec![])),
            URLItem::new(URLRecord::new("five", "five", "five", vec![])),
        ]
    }

    #[test]
    fn test_stateful_list() {
        let items = fix_url_items();

        let mut table = StatefulTable::with_items(items.as_slice());
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

    struct FixedFilter {
        matches: bool,
    }

    impl FixedFilter {
        fn new(matches: bool) -> FixedFilter {
            FixedFilter { matches }
        }
    }

    impl URLFilter for FixedFilter {
        fn matches(&self, _: &URLRecord) -> bool {
            return self.matches;
        }
    }

    #[test]
    fn test_prev_next_with_visibility() {
        let match_filter = FixedFilter::new(true);
        let do_not_match_filter = FixedFilter::new(false);

        let items = fix_url_items();
        let mut table = StatefulTable::with_items(items.as_slice());

        assert_eq!(table.items.len(), table.visible.len());

        table.items[1].filter(&do_not_match_filter);
        table.refresh_visible();
        assert_eq!(table.items.len() - 1, table.visible.len());

        table.items[0].filter(&do_not_match_filter);
        table.items[0].filter(&do_not_match_filter);
        table.items[2].filter(&do_not_match_filter);
        table.refresh_visible();
        assert_eq!(table.items.len() - 3, table.visible.len());

        table.items[0].filter(&match_filter);
        table.items[0].filter(&match_filter);
        table.refresh_visible();
        assert_eq!(table.items.len() - 2, table.visible.len());
    }
}
