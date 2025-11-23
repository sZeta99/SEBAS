use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    bookmark::Bookmark,
    group::{self, error::CRUDGroupError, Group},
};
pub trait IGroupAlias {
    fn to_alias(&self) -> Result<GroupAlias, CRUDGroupError>;
    fn form_alias(alias: GroupAlias, name: String) -> Result<Self, CRUDGroupError>;
}

/**
 * Alias for coorecly write the on the system the Group Object.
 */
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GroupAlias {
    pub(crate) bookmarks: Vec<Bookmark>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) modified_at: DateTime<Utc>,
}
impl IGroupAlias for Group {
    fn to_alias(&self) -> Result<GroupAlias, CRUDGroupError> {
        Ok(GroupAlias {
            bookmarks: self
                .get_bookmarks()
                .values()
                .map(|v: &Bookmark| v.clone())
                .collect(),
            created_at: self.get_created_time().clone(),
            modified_at: self.get_modified_time().clone(),
        })
    }

    fn form_alias(alias: GroupAlias, name: String) -> Result<Self, CRUDGroupError> {
        let mut group_alias = Group::new(name);
        group_alias = group_alias.map_err(|e| CRUDGroupError::AliasFaild(e.to_string));
    }
}
