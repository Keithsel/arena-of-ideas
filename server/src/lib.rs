pub mod ability;
pub mod arena;
pub mod arena_leaderboard;
pub mod arena_pool;
pub mod auction;
pub mod base_unit;
pub mod battle;
pub mod daily_state;
pub mod daily_updater;
pub mod fused_unit;
pub mod global_data;
pub mod global_event;
pub mod global_settings;
pub mod house;
pub mod incubator;
pub mod inflating_number;
pub mod items;
pub mod meta_shop;
pub mod migration;
pub mod player;
pub mod player_stats;
pub mod player_tag;
pub mod quest;
pub mod reward;
pub mod status;
pub mod team;
pub mod trade;
pub mod unit_balance;
pub mod wallet;

use std::str::FromStr;

pub use ability::*;
use anyhow::Context;
pub use arena::*;
pub use arena_leaderboard::*;
pub use arena_pool::*;
pub use auction::*;
pub use base_unit::*;
pub use battle::*;
pub use daily_state::*;
use daily_updater::daily_timer_init;
pub use fused_unit::*;
pub use global_data::*;
pub use global_event::*;
pub use global_settings::*;
pub use house::*;
pub use incubator::*;
pub use inflating_number::*;
pub use items::*;
pub use itertools::Itertools;
pub use meta_shop::*;
pub use player::*;
pub use player_stats::*;
pub use player_tag::*;
pub use quest::*;
pub use rand::{distributions::Alphanumeric, seq::IteratorRandom, Rng};
pub use reward::*;
pub use spacetimedb::{eprintln, println};
pub use spacetimedb::{Identity, ReducerContext, SpacetimeType, Table, Timestamp};
pub use status::*;
pub use team::*;
pub use trade::*;
pub use unit_balance::*;
pub use wallet::*;

trait StrContext<T> {
    fn context_str(self, str: &'static str) -> Result<T, String>;
    fn with_context_str<F>(self, f: F) -> Result<T, String>
    where
        F: FnOnce() -> String;
}

impl<T> StrContext<T> for Option<T> {
    fn context_str(self, str: &'static str) -> Result<T, String> {
        self.context(str).map_err(|e| e.to_string())
    }

    fn with_context_str<F>(self, f: F) -> Result<T, String>
    where
        F: FnOnce() -> String,
    {
        self.with_context(f).map_err(|e| e.to_string())
    }
}

pub fn next_id(ctx: &ReducerContext) -> u64 {
    GlobalData::next_id(ctx)
}

#[derive(SpacetimeType, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum GameMode {
    #[default]
    ArenaNormal = 0,
    ArenaRanked = 1,
    ArenaConst = 2,
}

const ADMIN_IDENTITY_HEX: &str = "c2000d3d36c3162dd302f78b29d2e3b78af2e0d9310cbe8fe9d75af5e9c393d0";
pub fn is_admin(identity: &Identity) -> Result<bool, String> {
    Ok(Identity::from_str(ADMIN_IDENTITY_HEX)
        .map_err(|e| e.to_string())?
        .eq(identity))
}

pub trait AdminCheck {
    fn is_admin(self) -> Result<(), String>;
}

impl AdminCheck for &ReducerContext {
    fn is_admin(self) -> Result<(), String> {
        if is_admin(&self.sender)? {
            Ok(())
        } else {
            Err("Need admin access".to_owned())
        }
    }
}

#[spacetimedb::reducer(init)]
fn init(ctx: &ReducerContext) -> Result<(), String> {
    GlobalData::init(ctx);
    Ok(())
}

#[inline]
pub fn default<T: Default>() -> T {
    Default::default()
}

#[spacetimedb::reducer]
fn cleanup(ctx: &ReducerContext) -> Result<(), String> {
    ctx.is_admin()?;
    TPlayer::cleanup(ctx);
    Ok(())
}
