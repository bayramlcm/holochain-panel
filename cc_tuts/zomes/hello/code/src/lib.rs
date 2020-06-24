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
    fn create_user(user_input: UserEntry) -> ZomeApiResult<User> {
        user::handlers::create_user(user_input)
    }

    #[zome_fn("hc_public")]
    fn get_user(id: Address) -> ZomeApiResult<User> {
        user::handlers::get_user(id)
    }

    #[zome_fn("hc_public")]
    fn update_user(id: Address, user_input: UserEntry) -> ZomeApiResult<User> {
        user::handlers::update_user(id, user_input)
    }

    #[zome_fn("hc_public")]
    fn remove_user(id: Address) -> ZomeApiResult<Address> {
        user::handlers::remove_user(id)
    }

    #[zome_fn("hc_public")]
    fn get_all_user() -> ZomeApiResult<Vec<User>> {
        user::handlers::get_all_user()
    }
}
