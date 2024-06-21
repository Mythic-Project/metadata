pub const PREFIX: &[u8] = b"mythic_metadata";
pub const COUNTER: &[u8] = b"counter";
pub const METADATA_KEY: &[u8] = b"metadata_key";
pub const METADATA: &[u8] = b"metadata";

pub const MAX_NAME_LEN: usize = 4 + 50;
pub const MAX_LABEL_LEN: usize = 4 + 30;
pub const MAX_DESCRIPTION_LEN: usize = 4 + 100;
pub const MAX_CONTENT_TYPE_LEN: usize = 4 + 50;
pub const MAX_VALUE_LEN: usize = 10000;
pub const MAX_COLLECTIONS_PER_METADATA: usize = 100;
pub const MAX_ITEMS_PER_COLLECTION: usize = 100;
