use crate::interactive::table::{StatefulTable, TableItem};
use crate::interactive::url_table_item::URLItem;
use bookmark_lib::Registry;
use bookmark_lib::types::URLRecord;

pub fn get_selected_item_id(table: &mut StatefulTable<URLItem>) -> Option<String> {
    table.state.selected()
        .map_or(None, |selected_index| {
            Some(table.visible[selected_index].id())
        })
}

pub fn get_selected_item<R: Registry>(registry: &R, table: &mut StatefulTable<URLItem>) -> Result<Option<URLRecord>, Box<dyn std::error::Error>> {
    Ok(get_selected_item_id(table)
        .map_or(None, |id: String| {
            registry.get_url(id)?
        }))
}

