use anchor_lang::prelude::*;

use crate::errors::*;

#[account]
#[derive(InitSpace)]
/// MetadataKey account defines a single metadata item
pub struct MetadataKey {
    /// Bump
    pub bump: u8,
    /// Authority of the MetadataKey namespace
    /// It allows authorities to create unique namespaces for metadata keys
    pub namespace_authority: Pubkey,

    /// Name of the MetadataKey
    /// It must be unique within the namespace authority
    #[max_len(30)]
    pub name: String,

    /// User friendly label of the MetadataKey
    #[max_len(50)]
    pub label: String,

    /// Description of the MetadataKey
    #[max_len(100)]
    pub description: String,

    /// The type of the metadata described by the key
    /// e.g. string, number, image, metadata, metadata-collection etc.
    #[max_len(20)]
    pub content_type: String,
}

impl MetadataKey {
    pub fn size() -> usize {
        8 + MetadataKey::INIT_SPACE
    }

    pub fn validate(name: &str, label: &str, description: &str, content_type: &str) -> Result<()> {
        if name.len() > 30 || label.len() > 50 || description.len() > 100 || content_type.len() > 20
        {
            return err!(DaoMetadataError::InvalidMetadataKeyField);
        }

        Ok(())
    }
}
