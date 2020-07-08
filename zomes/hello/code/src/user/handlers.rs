use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        entry::Entry,
    },
    holochain_persistence_api::cas::content::{
        Address,
        AddressableContent
    },
    prelude::*,
};
use holochain_anchors::anchor;
use crate::user::{
    UserEntry,
    User,
    USER_ENTRY_NAME,
    USERS_ANCHOR_TYPE,
    USERS_ANCHOR_TEXT,
    USER_LINK_TYPE
};

fn users_anchor() -> ZomeApiResult<Address> {
    anchor(USERS_ANCHOR_TYPE.to_string(), USERS_ANCHOR_TEXT.to_string())
}

pub fn user_add(user_entry: UserEntry) -> ZomeApiResult<User> {
    let entry = Entry::App(USER_ENTRY_NAME.into(), user_entry.clone().into());
    let address = hdk::commit_entry(&entry)?;
    hdk::link_entries(&users_anchor()?, &address, USER_LINK_TYPE, "")?;
    User::new(address, user_entry)
}

pub fn user_get(id: Address) -> ZomeApiResult<User> {
    let user: UserEntry = hdk::utils::get_as_type(id.clone())?;
    User::new(id, user)
}

pub fn user_update(id: Address, user_input: UserEntry) -> ZomeApiResult<User> {
    let address = match hdk::get_entry(&id.clone())? {
        None => id.clone(),
        Some(entry) => entry.address()
    };
    hdk::update_entry(Entry::App(USER_ENTRY_NAME.into(), user_input.clone().into()), &address)?;
    User::new(id, user_input)
}

pub fn user_delete(id: Address) -> ZomeApiResult<Address> {
    hdk::remove_link(&users_anchor()?, &id, USER_LINK_TYPE, "")?;
    hdk::remove_entry(&id)
}

pub fn user_get_all() -> ZomeApiResult<Vec<User>> {
    hdk::get_links_and_load(&users_anchor()?, LinkMatch::Exactly(USER_LINK_TYPE), LinkMatch::Any)
        .map(|user_list|{
            user_list.into_iter()
                .filter_map(Result::ok)
                .flat_map(|entry|{
                    let id = entry.address();
                    hdk::debug(format!("list_entry{:?}", entry)).ok();
                    user_get(id)
                }).collect()
        })
}

