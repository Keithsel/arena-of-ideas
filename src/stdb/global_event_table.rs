// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::global_event_type::GlobalEvent;
use super::t_global_event_type::TGlobalEvent;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `global_event`.
///
/// Obtain a handle from the [`GlobalEventTableAccess::global_event`] method on [`super::RemoteTables`],
/// like `ctx.db.global_event()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.global_event().on_insert(...)`.
pub struct GlobalEventTableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<TGlobalEvent>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `global_event`.
///
/// Implemented for [`super::RemoteTables`].
pub trait GlobalEventTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`GlobalEventTableHandle`], which mediates access to the table `global_event`.
    fn global_event(&self) -> GlobalEventTableHandle<'_>;
}

impl GlobalEventTableAccess for super::RemoteTables {
    fn global_event(&self) -> GlobalEventTableHandle<'_> {
        GlobalEventTableHandle {
            imp: self.imp.get_table::<TGlobalEvent>("global_event"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct GlobalEventInsertCallbackId(__sdk::callbacks::CallbackId);
pub struct GlobalEventDeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for GlobalEventTableHandle<'ctx> {
    type Row = TGlobalEvent;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = TGlobalEvent> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = GlobalEventInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> GlobalEventInsertCallbackId {
        GlobalEventInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: GlobalEventInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = GlobalEventDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> GlobalEventDeleteCallbackId {
        GlobalEventDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: GlobalEventDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

pub struct GlobalEventUpdateCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::TableWithPrimaryKey for GlobalEventTableHandle<'ctx> {
    type UpdateCallbackId = GlobalEventUpdateCallbackId;

    fn on_update(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row, &Self::Row) + Send + 'static,
    ) -> GlobalEventUpdateCallbackId {
        GlobalEventUpdateCallbackId(self.imp.on_update(Box::new(callback)))
    }

    fn remove_on_update(&self, callback: GlobalEventUpdateCallbackId) {
        self.imp.remove_on_update(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<TGlobalEvent>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_with_primary_key::<u64>(
        raw_updates,
        |row: &TGlobalEvent| &row.id,
    )
    .context("Failed to parse table update for table \"global_event\"")
}

/// Access to the `id` unique index on the table `global_event`,
/// which allows point queries on the field of the same name
/// via the [`GlobalEventIdUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.global_event().id().find(...)`.
pub struct GlobalEventIdUnique<'ctx> {
    imp: __sdk::client_cache::UniqueConstraint<TGlobalEvent, u64>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> GlobalEventTableHandle<'ctx> {
    /// Get a handle on the `id` unique index on the table `global_event`.
    pub fn id(&self) -> GlobalEventIdUnique<'ctx> {
        GlobalEventIdUnique {
            imp: self.imp.get_unique_constraint::<u64>("id", |row| &row.id),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> GlobalEventIdUnique<'ctx> {
    /// Find the subscribed row whose `id` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &u64) -> Option<TGlobalEvent> {
        self.imp.find(col_val)
    }
}