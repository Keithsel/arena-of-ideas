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
pub struct IncubatorFavoriteSet {
    pub id: u64,
}

impl __sdk::spacetime_module::InModule for IncubatorFavoriteSet {
    type Module = super::RemoteModule;
}

pub struct IncubatorFavoriteSetCallbackId(__sdk::callbacks::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `incubator_favorite_set`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait incubator_favorite_set {
    /// Request that the remote module invoke the reducer `incubator_favorite_set` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_incubator_favorite_set`] callbacks.
    fn incubator_favorite_set(&self, id: u64) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `incubator_favorite_set`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`IncubatorFavoriteSetCallbackId`] can be passed to [`Self::remove_on_incubator_favorite_set`]
    /// to cancel the callback.
    fn on_incubator_favorite_set(
        &self,
        callback: impl FnMut(&super::EventContext, &u64) + Send + 'static,
    ) -> IncubatorFavoriteSetCallbackId;
    /// Cancel a callback previously registered by [`Self::on_incubator_favorite_set`],
    /// causing it not to run in the future.
    fn remove_on_incubator_favorite_set(&self, callback: IncubatorFavoriteSetCallbackId);
}

impl incubator_favorite_set for super::RemoteReducers {
    fn incubator_favorite_set(&self, id: u64) -> __anyhow::Result<()> {
        self.imp
            .call_reducer("incubator_favorite_set", IncubatorFavoriteSet { id })
    }
    fn on_incubator_favorite_set(
        &self,
        mut callback: impl FnMut(&super::EventContext, &u64) + Send + 'static,
    ) -> IncubatorFavoriteSetCallbackId {
        IncubatorFavoriteSetCallbackId(self.imp.on_reducer::<IncubatorFavoriteSet>(
            "incubator_favorite_set",
            Box::new(
                move |ctx: &super::EventContext, args: &IncubatorFavoriteSet| {
                    callback(ctx, &args.id)
                },
            ),
        ))
    }
    fn remove_on_incubator_favorite_set(&self, callback: IncubatorFavoriteSetCallbackId) {
        self.imp
            .remove_on_reducer::<IncubatorFavoriteSet>("incubator_favorite_set", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `incubator_favorite_set`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_incubator_favorite_set {
    /// Set the call-reducer flags for the reducer `incubator_favorite_set` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn incubator_favorite_set(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_incubator_favorite_set for super::SetReducerFlags {
    fn incubator_favorite_set(&self, flags: __ws::CallReducerFlags) {
        self.imp
            .set_call_reducer_flags("incubator_favorite_set", flags);
    }
}