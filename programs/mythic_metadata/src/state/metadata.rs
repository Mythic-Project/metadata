use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
/// MetadataItem defines a single metadata item identified by its MetadataKey
pub struct MetadataItem {
    /// The Metadata Key Id
    pub metadata_key_id: u64,

    /// The slot when the value was last updated
    pub update_slot: u64,

    /// Serialized metadata item value
    #[max_len(MAX_VALUE_LEN)]
    pub value: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct MetadataCollection {
    /// The Metadata Key  Id
    pub metadata_key_id: u64,

    /// The slot when the collection was last updated
    /// The collection update slot is max(update_slot) for all its metadata items
    pub update_slot: u64,

    /// The authority that can update the collection metadata items
    /// Separate update instructions can be invoked to add/revoke specific collection's update_authority
    pub update_authority: Option<Pubkey>,

    /// Metadata items of the collection
    #[max_len(10)]
    pub items: Vec<MetadataItem>,
}

#[account]
#[derive(InitSpace)]
pub struct Metadata {
    /// Bump
    pub bump: u8,
    /// The Metadata Key numeric Id
    pub metadata_key_id: u64,
    /// The subject described by the metadata (e.g. a DAO, NFT, a program etc.)
    pub subject: Pubkey,

    /// The authority which issued (created) the Metadata account and owns it
    /// Note: The authority is embedded in the PDA seeds and cannot be changed
    /// If a new authority is required then a new Metadata account must be created
    ///
    /// Metadata can be self issued by the subject or issued by a third party
    /// For example a DAO can issue metadata about itself using the DAO's authority
    /// Or external authority can issue claims, certifications etc. about the DAO
    ///
    /// TODO:
    /// - Should is also be allowed to close the account?
    pub issuing_authority: Pubkey,

    /// The default update authority for all the collections (usually issuing_authority)
    /// Note: The authority can be overridden at the collection level
    pub update_authority: Pubkey,

    /// A set of metadata collections
    #[max_len(MAX_COLLECTIONS_PER_METADATA)]
    pub collections: Vec<MetadataCollection>,
}

impl Metadata {
    pub fn size() -> usize {
        8 + Metadata::INIT_SPACE
    }

    pub fn validate(&self) -> Result<()> {
        if self.collections.len() > MAX_COLLECTIONS_PER_METADATA {
            return err!(MythicMetadataError::MetadataCollectionFull);
        }
        Ok(())
    }
}
