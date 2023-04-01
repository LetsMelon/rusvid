use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, Serialize)]
pub enum ItemStatus {
    #[default]
    Pending,
    Processing,
    Finish,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct ItemList {
    pub list: HashMap<String, ItemStatus>,
}

pub type SharedItemList = Arc<RwLock<ItemList>>;
