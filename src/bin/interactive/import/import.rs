use std::{sync::mpsc, collections::HashSet};

use bookmark_lib::{import::{ImportItem}, Registry, filters::Filter};
use termion::event::{Key};

use crate::interactive::{table::{StatefulTable, TableItem}, event::Event};

use super::{import_table_item::ImportTableItem};



pub struct ImportsTable {
    signal_sender: mpsc::Sender<Event<Key>>,
    registry: Box<dyn Registry>,
    table: StatefulTable<ImportTableItem>,
    columns: Vec<String>,
    // filter: Option<Box<dyn Filter>>,
    // sort_cfg: Option<SortConfig>,

    tables_stack: Vec<StatefulTable<ImportTableItem>>,
    selected_imports: HashSet<String>,
}

impl ImportsTable {
    pub fn new(
        sender: mpsc::Sender<Event<Key>>,
        registry: Box<dyn Registry>,
        mut imports: Vec<ImportItem>,
    ) -> Result<ImportsTable, Box<dyn std::error::Error>> {
        let default_columns = vec!["Name".to_string(), "URL".to_string()];

        imports.sort();
        let table_items: Vec<ImportTableItem> = imports.into_iter().map(|imp| imp.into()).collect();

        let table = StatefulTable::with_items(table_items);

        // TODO: Store Vec of selected ids? As a top level state? This way we save
        // ourselves the pain of having self referential mutable data structures and
        // we do not store selection state in ImportItem iteself...

        Ok(ImportsTable {
            signal_sender: sender,
            registry,
            table: table,
            // filter: None,
            // sort_cfg: None,
            columns: default_columns,

            tables_stack: vec![],
            selected_imports: HashSet::new(),
        })
    }

    pub fn next(&mut self) {
        self.table.next()
    }

    pub fn previous(&mut self) {
        self.table.previous()
    }

    pub fn unselect(&mut self) {
        self.table.unselect()
    }

    pub fn table(&mut self) -> &mut StatefulTable<ImportTableItem> {
        &mut self.table
    }

    pub fn columns(&self) -> Vec<&'static str> {
        vec!["Type", "Name", "URL", "Selected"]
    }

    // pub fn get_selected(&self) -> Result<Option<URLRecord>, Box<dyn std::error::Error>> {
    //     let selected_id = self.get_selected_id();
    //     if selected_id.is_none() {
    //         return Ok(None);
    //     }

    //     let url_record = self.registry.get_url(&selected_id.unwrap())?;

    //     Ok(url_record)
    // }

    pub fn open(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // For URL select it
        // For folder open it

        match self.table.state.selected() {
            Some(id) => match &mut self.table.items[id] {
                ImportTableItem::URL(url) => {
                    let import_id = url.inner.id.to_string();
                    if self.selected_imports.contains(&import_id) {
                        self.selected_imports.remove(&import_id);
                        self.table.items[id].select(false);
                    } else {
                        self.selected_imports.insert(import_id);
                        self.table.items[id].select(true);
                    }
                    Ok(())
                },
                ImportTableItem::Folder(folder) => {
                    // We create new table with items from folder and replace
                    // it in current view, moving the old one to stack to 
                    // reclaim it later if user goes back.
                    let mut new_items = folder.inner.children.clone();
                    new_items.sort();
                    let new_table = StatefulTable::with_items(
                            new_items
                            .into_iter()
                            .map(|imp| {
                                let mut item: ImportTableItem = imp.into();
                                if self.selected_imports.contains(&item.id()) {
                                    item.select(true);
                                }
                                item
                            })
                            .collect()
                    );

                    let old_table = std::mem::replace(&mut self.table, new_table);

                    self.tables_stack.push(old_table);
                    Ok(())
                },
            },
            None => Ok(()),
        }
    }

    pub fn exit_folder(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(table) = self.tables_stack.pop() {
            self.table = table;
        }

        Ok(())
    }

    fn get_selected_id(&self) -> Option<String> {
        self.table
            .state
            .selected()
            .map(|index| self.table.items[index].id())
    }
}

// impl BookmarksTable {
//     pub fn new(
//         sender: mpsc::Sender<Event<Key>>,
//         registry: Box<dyn Registry>,
//     ) -> Result<BookmarksTable, Box<dyn std::error::Error>> {
//         let default_columns = default_columns();

//         let items: Vec<URLItem> =
//             URLItem::from_vec(registry.list_urls(None, None)?, Some(&default_columns));
//         let table = StatefulTable::with_items(items);

//         Ok(BookmarksTable {
//             signal_sender: sender,
//             registry,
//             table,
//             filter: None,
//             sort_cfg: None,
//             columns: default_columns,
//         })
//     }
// }