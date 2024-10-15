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
pub struct UnitBalanceVoteArgs {
    pub unit: String,
    pub vote: i32,
}

impl Reducer for UnitBalanceVoteArgs {
    const REDUCER_NAME: &'static str = "unit_balance_vote";
}

#[allow(unused)]
pub fn unit_balance_vote(unit: String, vote: i32) {
    UnitBalanceVoteArgs { unit, vote }.invoke();
}

#[allow(unused)]
pub fn on_unit_balance_vote(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &String, &i32) + Send + 'static,
) -> ReducerCallbackId<UnitBalanceVoteArgs> {
    UnitBalanceVoteArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let UnitBalanceVoteArgs { unit, vote } = __args;
        __callback(__identity, __addr, __status, unit, vote);
    })
}

#[allow(unused)]
pub fn once_on_unit_balance_vote(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &String, &i32) + Send + 'static,
) -> ReducerCallbackId<UnitBalanceVoteArgs> {
    UnitBalanceVoteArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let UnitBalanceVoteArgs { unit, vote } = __args;
        __callback(__identity, __addr, __status, unit, vote);
    })
}

#[allow(unused)]
pub fn remove_on_unit_balance_vote(id: ReducerCallbackId<UnitBalanceVoteArgs>) {
    UnitBalanceVoteArgs::remove_on_reducer(id);
}