use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;

#[account]
#[derive(InitSpace)]
/// MetadataKey account defines a single metadata value
pub struct MetadataKey {
    /// Id
    pub id: u64,

    /// Authority of the MetadataKey namespace
    /// It allows authorities to create unique namespaces for metadata keys
    pub namespace_authority: Pubkey,

    /// Name of the metadata value represented by the MetadataKey
    #[max_len(MAX_NAME_LEN)]
    pub name: String,

    /// User friendly label of the value represented by the MetadataKey
    #[max_len(MAX_LABEL_LEN)]
    pub label: String,

    /// Description of the value represented by the MetadataKey
    #[max_len(MAX_DESCRIPTION_LEN)]
    pub description: String,

    /// The type of the metadata described by the key
    /// e.g. string, number, image, metadata, metadata-collection etc.
    #[max_len(MAX_CONTENT_TYPE_LEN)]
    pub content_type: String,

    /// Bump
    pub bump: u8,
}

impl MetadataKey {
    pub fn size() -> usize {
        8 + MetadataKey::INIT_SPACE
    }

    pub fn validate(name: &str, label: &str, description: &str, content_type: &str) -> Result<()> {
        if name.len() > MAX_NAME_LEN
            || label.len() > MAX_LABEL_LEN
            || description.len() > MAX_DESCRIPTION_LEN
            || content_type.len() > MAX_CONTENT_TYPE_LEN
        {
            return err!(MythicMetadataError::InvalidMetadataKey);
        }

        Ok(())
    }
}
