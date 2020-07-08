use hdk::{
    self, entry,
    entry_definition::ValidatingEntryType,
    from,
    holochain_core_types::{dna::entry_types::Sharing, time::Iso8601, time::Timeout},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
    link,
    prelude::*,
};
use holochain_json_derive::DefaultJson;
use serde_derive::{Deserialize, Serialize};

pub mod handlers;
pub mod validation;

const USER_ENTRY_NAME: &str = "user";
const USER_LINK_TYPE: &str = "user_link";
const USERS_ANCHOR_TYPE: &str = "users";
const USERS_ANCHOR_TEXT: &str = "users";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserEntry {
    name: String,
    mail: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: Address,
    created_at: Iso8601,
    name: String,
    mail: String,
}

fn timestamp(address: Address) -> ZomeApiResult<Iso8601> {
    let options = GetEntryOptions {
        status_request: StatusRequestKind::Initial,
        entry: false,
        headers: true,
        timeout: Timeout::new(10000),
    };
    let entry_result = hdk::get_entry_result(&address, options)?;
    match entry_result.result {
        GetEntryResultType::Single(entry) => Ok(entry.headers[0].timestamp().clone()),
        _ => unreachable!(),
    }
}

impl User {
    pub fn new(id: Address, user_entry: UserEntry) -> ZomeApiResult<User> {
        Ok(User {
            id: id.clone(),
            created_at: timestamp(id)?,
            name: user_entry.name,
            mail: user_entry.mail,
        })
    }
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: USER_ENTRY_NAME,
        description: "kullanıcı bilgileri",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<UserEntry>| {
            match validation_data
            {
                hdk::EntryValidationData::Create{entry, validation_data} =>
                {
                    validation::validate_entry_create(entry, validation_data)
                },
                hdk::EntryValidationData::Modify{new_entry, old_entry, old_entry_header, validation_data} =>
                {
                    validation::validate_entry_modify(new_entry, old_entry, old_entry_header, validation_data)
                },
                hdk::EntryValidationData::Delete{old_entry, old_entry_header, validation_data} =>
                {
                   validation::validate_entry_delete(old_entry, old_entry_header, validation_data)
                }
            }
        },
        links: [
            from!(
                holochain_anchors::ANCHOR_TYPE,
                link_type: USER_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |validation_data: hdk::LinkValidationData| {
                    match validation_data
                    {
                        hdk::LinkValidationData::LinkAdd{link, validation_data} =>
                        {
                            validation::validate_link_add(link, validation_data)
                        },
                        hdk::LinkValidationData::LinkRemove{link, validation_data} =>
                        {
                            validation::validate_link_remove(link, validation_data)
                        }
                    }
                }
            )
        ]
    )
}
