use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;

#[derive(Debug)]
#[account]
/// MetadataKey account defines a single metadata value
pub struct MetadataKey {
    /// Id
    pub id: u64,

    /// Authority of the MetadataKey namespace
    /// It allows authorities to create unique namespaces for metadata keys
    pub namespace_authority: Pubkey,

    /// Name of the metadata value represented by the MetadataKey
    pub name: String,

    /// User friendly label of the value represented by the MetadataKey
    pub label: String,

    /// Description of the value represented by the MetadataKey
    pub description: String,

    /// The type of the metadata described by the key
    /// e.g. string, number, image, metadata, metadata-collection etc.
    pub content_type: String,

    /// Bump
    pub bump: u8,
}

impl MetadataKey {
    pub fn size(name: &str, label: &str, description: &str, content_type: &str) -> usize {
        8 + // Anchor discriminator
        8 + // ID
        32 + // Namespace Authority
        4 + name.len() + // Name
        4 + label.len() + // Label
        4 + description.len() + // Description
        4 + content_type.len() + // Content Type
        1 // bump
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
