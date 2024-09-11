// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address, ScheduleAt,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CraftHeroArgs {
    pub base: String,
}

impl Reducer for CraftHeroArgs {
    const REDUCER_NAME: &'static str = "craft_hero";
}

#[allow(unused)]
pub fn craft_hero(base: String) {
    CraftHeroArgs { base }.invoke();
}

#[allow(unused)]
pub fn on_craft_hero(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &String) + Send + 'static,
) -> ReducerCallbackId<CraftHeroArgs> {
    CraftHeroArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let CraftHeroArgs { base } = __args;
        __callback(__identity, __addr, __status, base);
    })
}

#[allow(unused)]
pub fn once_on_craft_hero(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &String) + Send + 'static,
) -> ReducerCallbackId<CraftHeroArgs> {
    CraftHeroArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let CraftHeroArgs { base } = __args;
        __callback(__identity, __addr, __status, base);
    })
}

#[allow(unused)]
pub fn remove_on_craft_hero(id: ReducerCallbackId<CraftHeroArgs>) {
    CraftHeroArgs::remove_on_reducer(id);
}