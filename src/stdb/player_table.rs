// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::t_player_type::TPlayer;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `player`.
///
/// Obtain a handle from the [`PlayerTableAccess::player`] method on [`super::RemoteTables`],
/// like `ctx.db.player()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.player().on_insert(...)`.
pub struct PlayerTableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<TPlayer>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `player`.
///
/// Implemented for [`super::RemoteTables`].
pub trait PlayerTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`PlayerTableHandle`], which mediates access to the table `player`.
    fn player(&self) -> PlayerTableHandle<'_>;
}

impl PlayerTableAccess for super::RemoteTables {
    fn player(&self) -> PlayerTableHandle<'_> {
        PlayerTableHandle {
            imp: self.imp.get_table::<TPlayer>("player"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct PlayerInsertCallbackId(__sdk::callbacks::CallbackId);
pub struct PlayerDeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for PlayerTableHandle<'ctx> {
    type Row = TPlayer;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = TPlayer> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = PlayerInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> PlayerInsertCallbackId {
        PlayerInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: PlayerInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = PlayerDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> PlayerDeleteCallbackId {
        PlayerDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: PlayerDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

pub struct PlayerUpdateCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::TableWithPrimaryKey for PlayerTableHandle<'ctx> {
    type UpdateCallbackId = PlayerUpdateCallbackId;

    fn on_update(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row, &Self::Row) + Send + 'static,
    ) -> PlayerUpdateCallbackId {
        PlayerUpdateCallbackId(self.imp.on_update(Box::new(callback)))
    }

    fn remove_on_update(&self, callback: PlayerUpdateCallbackId) {
        self.imp.remove_on_update(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<TPlayer>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_with_primary_key::<u64>(
        raw_updates,
        |row: &TPlayer| &row.id,
    )
    .context("Failed to parse table update for table \"player\"")
}

/// Access to the `id` unique index on the table `player`,
/// which allows point queries on the field of the same name
/// via the [`PlayerIdUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.player().id().find(...)`.
pub struct PlayerIdUnique<'ctx> {
    imp: __sdk::client_cache::UniqueConstraint<TPlayer, u64>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> PlayerTableHandle<'ctx> {
    /// Get a handle on the `id` unique index on the table `player`.
    pub fn id(&self) -> PlayerIdUnique<'ctx> {
        PlayerIdUnique {
            imp: self.imp.get_unique_constraint::<u64>("id", |row| &row.id),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> PlayerIdUnique<'ctx> {
    /// Find the subscribed row whose `id` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &u64) -> Option<TPlayer> {
        self.imp.find(col_val)
    }
}

/// Access to the `name` unique index on the table `player`,
/// which allows point queries on the field of the same name
/// via the [`PlayerNameUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.player().name().find(...)`.
pub struct PlayerNameUnique<'ctx> {
    imp: __sdk::client_cache::UniqueConstraint<TPlayer, String>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> PlayerTableHandle<'ctx> {
    /// Get a handle on the `name` unique index on the table `player`.
    pub fn name(&self) -> PlayerNameUnique<'ctx> {
        PlayerNameUnique {
            imp: self
                .imp
                .get_unique_constraint::<String>("name", |row| &row.name),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> PlayerNameUnique<'ctx> {
    /// Find the subscribed row whose `name` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &String) -> Option<TPlayer> {
        self.imp.find(col_val)
    }
}