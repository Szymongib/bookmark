use tui::widgets::TableState;

pub trait TableItem {
    fn row(&self) -> &Vec<String>;
    fn id(&self) -> String;
}

pub struct StatefulTable<T> {
    pub items: Vec<T>,
    pub state: TableState,
}

impl<T> StatefulTable<T> {
    pub fn with_items(items: Vec<T>) -> StatefulTable<T> {
        StatefulTable {
            state: TableState::default(),
            items: items,
        }
    }

    pub fn override_items(&mut self, items: Vec<T>) {
        self.items = items;
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
    use crate::interactive::table::StatefulTable;

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

//     pub fn items(&self) -> Result<Vec<T>, Box<dyn std::error::Error>> {
//         self.source.get_items()
//     }
//
//     pub fn set_filter(&mut self, filter: F<T>) {
//         self.filter = Some(filter)
//     }
//
//     pub fn refresh_visible(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//         if self.filter.is_none() {
//             self.visible = self.items()?;
//             return Ok(())
//         }
//
//         self.visible.clear();
//         let items = self.source.get_items()?;
//
//         let filter = &self.filter.unwrap();
//         for i in items {
//             if filter.apply(i) {
//                 self.visible.push(i.clone())
//             }
//         }
//
//         Ok(())
//     }

//
// #[cfg(test)]
// mod test {
//     use crate::interactive::table::StatefulTable;
//     use crate::interactive::url_table_item::URLItem;
//     use bookmark_lib::record_filter::URLFilter;
//     use bookmark_lib::types::URLRecord;
//
//     fn fix_url_items() -> Vec<URLItem> {
//         vec![
//             URLItem::new(URLRecord::new("one", "one", "one", vec![])),
//             URLItem::new(URLRecord::new("two", "two", "two", vec![])),
//             URLItem::new(URLRecord::new("three", "three", "three", vec![])),
//             URLItem::new(URLRecord::new("four", "four", "four", vec![])),
//             URLItem::new(URLRecord::new("five", "five", "five", vec![])),
//         ]
//     }
//
//     #[test]
//     fn test_stateful_list() {
//         let items = fix_url_items();
//
//         let mut table = StatefulTable::with_items(items.as_slice());
//         assert_eq!(table.state.selected(), None);
//
//         table.next();
//         assert_eq!(table.state.selected(), Some(0));
//
//         table.previous();
//         assert_eq!(table.state.selected(), Some(4));
//
//         table.next();
//         table.next();
//         table.next();
//         table.next();
//         table.next();
//         table.next();
//         assert_eq!(table.state.selected(), Some(0));
//
//         table.unselect();
//         assert_eq!(table.state.selected(), None);
//     }
//
//     struct FixedFilter {
//         matches: bool,
//     }
//
//     impl FixedFilter {
//         fn new(matches: bool) -> FixedFilter {
//             FixedFilter { matches }
//         }
//     }
//
//     impl URLFilter for FixedFilter {
//         fn matches(&self, _: &URLRecord) -> bool {
//             return self.matches;
//         }
//     }
//
//     #[test]
//     fn test_prev_next_with_visibility() {
//         let match_filter = FixedFilter::new(true);
//         let do_not_match_filter = FixedFilter::new(false);
//
//         let items = fix_url_items();
//         let mut table = StatefulTable::with_items(items.as_slice());
//
//         assert_eq!(table.items.len(), table.visible.len());
//
//         table.items[1].filter(&do_not_match_filter);
//         table.refresh_visible();
//         assert_eq!(table.items.len() - 1, table.visible.len());
//
//         table.items[0].filter(&do_not_match_filter);
//         table.items[0].filter(&do_not_match_filter);
//         table.items[2].filter(&do_not_match_filter);
//         table.refresh_visible();
//         assert_eq!(table.items.len() - 3, table.visible.len());
//
//         table.items[0].filter(&match_filter);
//         table.items[0].filter(&match_filter);
//         table.refresh_visible();
//         assert_eq!(table.items.len() - 2, table.visible.len());
//     }
// }
