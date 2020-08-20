use crate::interactive::table::{StatefulTable, TableItem};
use crate::interactive::url_table_item::{URLItem, URLItemSource};
use bookmark_lib::Registry;
use bookmark_lib::types::URLRecord;

pub fn get_selected_item_id<R: Registry>(table: &mut StatefulTable<URLItemSource<R>, URLItem>) -> Option<String> {
    table.state.selected()
        .map_or(None, |selected_index| {
            Some(table.visible[selected_index].id())
        })
}

pub fn get_selected_item<R: Registry>(registry: &R, table: &mut StatefulTable<URLItemSource<R>, URLItem>) -> Result<Option<URLRecord>, Box<dyn std::error::Error>> {
    let id = get_selected_item_id(table);
    if id.is_none() { return Ok(None) }
    let url = registry.get_url(id.unwrap())?;
    Ok(url)
}

// TODO: consider moving to some lib
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
