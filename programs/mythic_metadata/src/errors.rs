use anchor_lang::prelude::*;

#[error_code]
pub enum MythicMetadataError {
    #[msg("Cannot increment ID")]
    CounterIdReachedMax,
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Metadata immutable")]
    ImmutableMetadata,
    #[msg("Invalid MetadataKey")]
    InvalidMetadataKey,
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
