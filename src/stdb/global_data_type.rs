// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct GlobalData {
    pub always_zero: u32,
    pub next_id: u64,
    pub game_version: String,
    pub last_sync: u64,
    pub initial_enemies: Vec<u64>,
}

impl __sdk::spacetime_module::InModule for GlobalData {
    type Module = super::RemoteModule;
}