// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::fused_unit_type::FusedUnit;
use super::t_team_type::TTeam;
use super::team_pool_type::TeamPool;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `team`.
///
/// Obtain a handle from the [`TeamTableAccess::team`] method on [`super::RemoteTables`],
/// like `ctx.db.team()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.team().on_insert(...)`.
pub struct TeamTableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<TTeam>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `team`.
///
/// Implemented for [`super::RemoteTables`].
pub trait TeamTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`TeamTableHandle`], which mediates access to the table `team`.
    fn team(&self) -> TeamTableHandle<'_>;
}

impl TeamTableAccess for super::RemoteTables {
    fn team(&self) -> TeamTableHandle<'_> {
        TeamTableHandle {
            imp: self.imp.get_table::<TTeam>("team"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct TeamInsertCallbackId(__sdk::callbacks::CallbackId);
pub struct TeamDeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for TeamTableHandle<'ctx> {
    type Row = TTeam;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = TTeam> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = TeamInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> TeamInsertCallbackId {
        TeamInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: TeamInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = TeamDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> TeamDeleteCallbackId {
        TeamDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: TeamDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

pub struct TeamUpdateCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::TableWithPrimaryKey for TeamTableHandle<'ctx> {
    type UpdateCallbackId = TeamUpdateCallbackId;

    fn on_update(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row, &Self::Row) + Send + 'static,
    ) -> TeamUpdateCallbackId {
        TeamUpdateCallbackId(self.imp.on_update(Box::new(callback)))
    }

    fn remove_on_update(&self, callback: TeamUpdateCallbackId) {
        self.imp.remove_on_update(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<TTeam>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_with_primary_key::<u64>(
        raw_updates,
        |row: &TTeam| &row.id,
    )
    .context("Failed to parse table update for table \"team\"")
}

/// Access to the `id` unique index on the table `team`,
/// which allows point queries on the field of the same name
/// via the [`TeamIdUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.team().id().find(...)`.
pub struct TeamIdUnique<'ctx> {
    imp: __sdk::client_cache::UniqueConstraint<TTeam, u64>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> TeamTableHandle<'ctx> {
    /// Get a handle on the `id` unique index on the table `team`.
    pub fn id(&self) -> TeamIdUnique<'ctx> {
        TeamIdUnique {
            imp: self.imp.get_unique_constraint::<u64>("id", |row| &row.id),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> TeamIdUnique<'ctx> {
    /// Find the subscribed row whose `id` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &u64) -> Option<TTeam> {
        self.imp.find(col_val)
    }
}
