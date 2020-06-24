#![feature(proc_macro_hygiene)]
use crate::user::{User, UserEntry};
use hdk::{
    entry_definition::ValidatingEntryType, error::ZomeApiResult,
    holochain_persistence_api::cas::content::Address,
};
use hdk_proc_macros::zome;
use serde_derive::{Deserialize, Serialize};
pub mod user;

#[zome]
mod users {
    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn valildate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn anchor_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }

    #[entry_def]
    fn user_def() -> ValidatingEntryType {
        user::definition()
    }

    #[zome_fn("hc_public")]
    fn user_add(user: UserEntry) -> ZomeApiResult<User> {
        user::handlers::user_add(user)
    }

    #[zome_fn("hc_public")]
    fn user_get(id: Address) -> ZomeApiResult<User> {
        user::handlers::user_get(id)
    }

    #[zome_fn("hc_public")]
    fn user_update(id: Address, user_input: UserEntry) -> ZomeApiResult<User> {
        user::handlers::user_update(id, user_input)
    }

    #[zome_fn("hc_public")]
    fn user_delete(id: Address) -> ZomeApiResult<Address> {
        user::handlers::user_delete(id)
    }

    #[zome_fn("hc_public")]
    fn user_get_all() -> ZomeApiResult<Vec<User>> {
        user::handlers::user_get_all()
    }
}
