use anchor_lang::prelude::*;

#[error_code]
pub enum DaoMetadataError {
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid MetadataKey field")]
    InvalidMetadataKeyField,
    #[msg("Metadata collection is full")]
    MetadataCollectionFull,
    #[msg("Metadata collection already exists")]
    MetadataCollectionAlreadyExists,
    #[msg("Metadata collection does not exist")]
    MetadataCollectionNonExistent,
    #[msg("Metadata item is full")]
    MetadataItemFull,
    #[msg("Metadata item already exists")]
    MetadataItemAlreadyExists,
}
