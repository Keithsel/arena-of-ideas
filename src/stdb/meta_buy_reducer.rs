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
pub struct MetaBuy {
    pub id: u64,
}

impl __sdk::spacetime_module::InModule for MetaBuy {
    type Module = super::RemoteModule;
}

pub struct MetaBuyCallbackId(__sdk::callbacks::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `meta_buy`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait meta_buy {
    /// Request that the remote module invoke the reducer `meta_buy` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_meta_buy`] callbacks.
    fn meta_buy(&self, id: u64) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `meta_buy`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`MetaBuyCallbackId`] can be passed to [`Self::remove_on_meta_buy`]
    /// to cancel the callback.
    fn on_meta_buy(
        &self,
        callback: impl FnMut(&super::EventContext, &u64) + Send + 'static,
    ) -> MetaBuyCallbackId;
    /// Cancel a callback previously registered by [`Self::on_meta_buy`],
    /// causing it not to run in the future.
    fn remove_on_meta_buy(&self, callback: MetaBuyCallbackId);
}

impl meta_buy for super::RemoteReducers {
    fn meta_buy(&self, id: u64) -> __anyhow::Result<()> {
        self.imp.call_reducer("meta_buy", MetaBuy { id })
    }
    fn on_meta_buy(
        &self,
        mut callback: impl FnMut(&super::EventContext, &u64) + Send + 'static,
    ) -> MetaBuyCallbackId {
        MetaBuyCallbackId(self.imp.on_reducer::<MetaBuy>(
            "meta_buy",
            Box::new(move |ctx: &super::EventContext, args: &MetaBuy| callback(ctx, &args.id)),
        ))
    }
    fn remove_on_meta_buy(&self, callback: MetaBuyCallbackId) {
        self.imp
            .remove_on_reducer::<MetaBuy>("meta_buy", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `meta_buy`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_meta_buy {
    /// Set the call-reducer flags for the reducer `meta_buy` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn meta_buy(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_meta_buy for super::SetReducerFlags {
    fn meta_buy(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("meta_buy", flags);
    }
}
