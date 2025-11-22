use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{bookmark::Bookmark, group::Group};
/**
 * Alias for coorecly write the on the system the Group Object.
 */
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GroupAlias {
    pub(crate) bookmarks: Vec<Bookmark>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) modified_at: DateTime<Utc>,
}
