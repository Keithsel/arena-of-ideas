// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct AdminGiveTagArgs {
    pub owner: u64,
    pub tag: String,
}

impl From<AdminGiveTagArgs> for super::Reducer {
    fn from(args: AdminGiveTagArgs) -> Self {
        Self::AdminGiveTag {
            owner: args.owner,
            tag: args.tag,
        }
    }
}

impl __sdk::InModule for AdminGiveTagArgs {
    type Module = super::RemoteModule;
}

pub struct AdminGiveTagCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `admin_give_tag`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait admin_give_tag {
    /// Request that the remote module invoke the reducer `admin_give_tag` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_admin_give_tag`] callbacks.
    fn admin_give_tag(&self, owner: u64, tag: String) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `admin_give_tag`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`AdminGiveTagCallbackId`] can be passed to [`Self::remove_on_admin_give_tag`]
    /// to cancel the callback.
    fn on_admin_give_tag(
        &self,
        callback: impl FnMut(&super::EventContext, &u64, &String) + Send + 'static,
    ) -> AdminGiveTagCallbackId;
    /// Cancel a callback previously registered by [`Self::on_admin_give_tag`],
    /// causing it not to run in the future.
    fn remove_on_admin_give_tag(&self, callback: AdminGiveTagCallbackId);
}

impl admin_give_tag for super::RemoteReducers {
    fn admin_give_tag(&self, owner: u64, tag: String) -> __anyhow::Result<()> {
        self.imp
            .call_reducer("admin_give_tag", AdminGiveTagArgs { owner, tag })
    }
    fn on_admin_give_tag(
        &self,
        mut callback: impl FnMut(&super::EventContext, &u64, &String) + Send + 'static,
    ) -> AdminGiveTagCallbackId {
        AdminGiveTagCallbackId(self.imp.on_reducer(
            "admin_give_tag",
            Box::new(move |ctx: &super::EventContext| {
                let super::EventContext {
                    event:
                        __sdk::Event::Reducer(__sdk::ReducerEvent {
                            reducer: super::Reducer::AdminGiveTag { owner, tag },
                            ..
                        }),
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, owner, tag)
            }),
        ))
    }
    fn remove_on_admin_give_tag(&self, callback: AdminGiveTagCallbackId) {
        self.imp.remove_on_reducer("admin_give_tag", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `admin_give_tag`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_admin_give_tag {
    /// Set the call-reducer flags for the reducer `admin_give_tag` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn admin_give_tag(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_admin_give_tag for super::SetReducerFlags {
    fn admin_give_tag(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("admin_give_tag", flags);
    }
}
