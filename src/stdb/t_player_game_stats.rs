// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use super::game_mode::GameMode;
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
pub struct TPlayerGameStats {
    pub id: u64,
    pub season: u32,
    pub owner: u64,
    pub mode: GameMode,
    pub runs: u32,
    pub floors: Vec<u32>,
    pub champion: u32,
    pub boss: u32,
}

impl TableType for TPlayerGameStats {
    const TABLE_NAME: &'static str = "TPlayerGameStats";
    type ReducerEvent = super::ReducerEvent;
}

impl TableWithPrimaryKey for TPlayerGameStats {
    type PrimaryKey = u64;
    fn primary_key(&self) -> &Self::PrimaryKey {
        &self.id
    }
}

impl TPlayerGameStats {
    #[allow(unused)]
    pub fn filter_by_id(id: u64) -> TableIter<Self> {
        Self::filter(|row| row.id == id)
    }
    #[allow(unused)]
    pub fn find_by_id(id: u64) -> Option<Self> {
        Self::find(|row| row.id == id)
    }
    #[allow(unused)]
    pub fn filter_by_season(season: u32) -> TableIter<Self> {
        Self::filter(|row| row.season == season)
    }
    #[allow(unused)]
    pub fn filter_by_owner(owner: u64) -> TableIter<Self> {
        Self::filter(|row| row.owner == owner)
    }
    #[allow(unused)]
    pub fn filter_by_runs(runs: u32) -> TableIter<Self> {
        Self::filter(|row| row.runs == runs)
    }
    #[allow(unused)]
    pub fn filter_by_champion(champion: u32) -> TableIter<Self> {
        Self::filter(|row| row.champion == champion)
    }
    #[allow(unused)]
    pub fn filter_by_boss(boss: u32) -> TableIter<Self> {
        Self::filter(|row| row.boss == boss)
    }
}