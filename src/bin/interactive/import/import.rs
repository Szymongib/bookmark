use std::{sync::mpsc, collections::HashSet};

use bookmark_lib::{import::{ImportItem}, Registry, filters::Filter};
use termion::event::{Key};

use crate::interactive::{table::{StatefulTable, TableItem}, event::Event};

use super::{import_table_item::{ImportTableItem, ImportFolderTableItem}};



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

            // TODO: Separate HashSet for Folders and URLs? Or HashSet of enums?
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

    pub fn open_or_select(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.table.state.selected() {
            // For URL default action will be to mark it as selected or unselect
            // For Folder we will open it.
            Some(id) => match &mut self.table.items[id] {
                ImportTableItem::URL(_) => {
                    self.select_item(id);
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

    /// selects item to import regardless it it is folder or URL
    pub fn toggle_selected(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.table.state.selected() {
            None => Ok(()),
            Some(id) => {
                self.select_item(id);
                Ok(())
            }
        }
    }

    fn select_item(&mut self, id: usize) {
        // TODO: I am not fully happy with this logic in regards to folders,
        // since ideally you could select the top folder and all its children
        // would be marked selected, then you could unselect just some of them.
        // It should be possible to implement it this way, tho it will require
        // some though how to make it resonably, and in a way it still feels
        // instant to user.

        let item = &mut self.table.items[id];
        let import_id = item.id();
        if self.selected_imports.contains(&import_id) {
            self.selected_imports.remove(&import_id);
            self.table.items[id].select(false);
        } else {
            self.selected_imports.insert(import_id);
            self.table.items[id].select(true);
        }
    }

    pub fn exit_folder(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(table) = self.tables_stack.pop() {
            self.table = table;
        }
        Ok(())
    }

    pub fn open_url(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.table.state.selected() {
            Some(id) => match &self.table.items[id] {
                ImportTableItem::URL(url) => {
                    open::that(url.inner.url.as_str())
                        .map_err(|err| -> Box<dyn std::error::Error> {
                            From::from(format!("failed to open URL in the browser: {}",err.to_string()))
                        })?;
                    Ok(())
                },
                ImportTableItem::Folder(_) => Ok(()),
            },
            None => Ok(()),
        }
    }

    fn get_selected_id(&self) -> Option<String> {
        self.table
            .state
            .selected()
            .map(|index| self.table.items[index].id())
    }
}
