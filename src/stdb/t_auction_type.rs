// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

use super::item_kind_type::ItemKind;

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct TAuction {
    pub item_id: u64,
    pub owner: u64,
    pub item_kind: ItemKind,
    pub price: i64,
}

impl __sdk::spacetime_module::InModule for TAuction {
    type Module = super::RemoteModule;
}