use std::sync::mpsc;

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
}

impl ImportsTable {
    pub fn new(
        sender: mpsc::Sender<Event<Key>>,
        registry: Box<dyn Registry>,
        imports: Vec<ImportItem>,
    ) -> Result<ImportsTable, Box<dyn std::error::Error>> {
        let default_columns = vec!["Name".to_string(), "URL".to_string()];

        let table_items: Vec<ImportTableItem> = imports.into_iter().map(|imp| imp.into()).collect();

        let table = StatefulTable::with_items(table_items);

        // TODO: Store Vec of selected ids? As a top level state? This way we save
        // ourselves the pain of having self referential mutable data structures and
        // we do not store selection state in ImportItem iteself...

        Ok(ImportsTable {
            signal_sender: sender,
            registry,
            table,
            // filter: None,
            // sort_cfg: None,
            columns: default_columns,
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
                    // TODO: select URL
                    Ok(())
                },
                ImportTableItem::Folder(folder) => {
                    self.table = StatefulTable::with_items(
                        folder.inner.children.clone()
                            .into_iter()
                            .map(|imp| imp.into())
                            .collect()
                    );
                    Ok(())
                },
            },
            None => Ok(()),
        }
    }

    fn refresh_items(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // let urls = match &self.filter {
        //     Some(f) => self.registry.list_urls(Some(f.as_ref()), self.sort_cfg)?,
        //     None => self.registry.list_urls(None, self.sort_cfg)?,
        // };

        // self.table
        //     .override_items(URLItem::from_vec(urls, Some(&self.columns)));
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