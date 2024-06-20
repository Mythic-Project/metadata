use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
/// MetadataItem defines a single metadata item identified by its MetadataKey
pub struct MetadataItem {
    /// The Metadata Key Id
    pub metadata_key_id: u64,

    /// The slot when the value was last updated
    pub update_slot: u64,

    /// Serialized metadata item value
    pub value: Vec<u8>,
}

impl MetadataItem {
    pub fn size(value: &[u8]) -> usize {
        8 + // metadata_key_id
        8 + // update_slot
        4 + value.len() // value
    }

    pub fn validate(&self) -> Result<()> {
        if self.value.len() > MAX_VALUE_LEN {
            return err!(MythicMetadataError::MetadataItemValueLenExceeded);
        }

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MetadataCollection {
    /// The Metadata Key  Id
    pub metadata_key_id: u64,

    /// The slot when the collection was last updated
    /// The collection update slot is max(update_slot) for all its metadata items
    pub update_slot: u64,

    /// The authority that can update the collection metadata items
    /// Separate update instructions can be invoked to add/revoke specific collection's update_authority
    /// If the collection level update authority is None then parent Metadata update_authority is used
    pub update_authority: Option<Pubkey>,

    pub items: Vec<MetadataItem>,
}

impl MetadataCollection {
    pub fn size(collection_items: &[MetadataItem]) -> usize {
        let collection_items_size = collection_items.iter().fold(0, |mut acc, collection_item| {
            let metadata_item_size = MetadataItem::size(&collection_item.value);
            acc += metadata_item_size;
            acc
        });

        8 + // metadata_key_id
        8 + // update_slot
        1 + 32 + // update_authority
        4 + collection_items_size // items
    }

    pub fn validate(&self) -> Result<()> {
        if self.items.len() > MAX_ITEMS_PER_COLLECTION {
            return err!(MythicMetadataError::MetadataItemFull);
        }

        for item in &self.items {
            item.validate()?;
        }

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MetadataRootCollection {
    /// The Metadata Key  Id
    pub metadata_key_id: u64,

    /// The slot when the collection was last updated
    /// The collection update slot is max(update_slot) for all its metadata items
    pub update_slot: u64,

    /// The default update authority for all the collections
    /// Note: The authority can be overridden at the collection level
    /// Setting the authority to None makes the Metadata immutable
    pub update_authority: Option<Pubkey>,

    pub items: Vec<MetadataItem>,

    pub collections: Vec<MetadataCollection>,
}

impl MetadataRootCollection {
    pub fn size(
        root_collection_items: &[MetadataItem],
        collections: &[MetadataCollection],
    ) -> usize {
        let root_collection_items_size =
            root_collection_items
                .iter()
                .fold(0, |mut acc, collection_item| {
                    let metadata_item_size = MetadataItem::size(&collection_item.value);
                    acc += metadata_item_size;
                    acc
                });

        let collections_size = collections.iter().fold(0, |mut acc, child_collection| {
            let child_collection_size = MetadataCollection::size(&child_collection.items);
            acc += child_collection_size;
            acc
        });

        8 + // metadata_key_id
        8 + // update_slot
        1 + 32 + // update_authority
        4 + root_collection_items_size + // items
        4 + collections_size // children
    }

    pub fn validate(&self) -> Result<()> {
        if self.items.len() > MAX_ITEMS_PER_COLLECTION {
            return err!(MythicMetadataError::MetadataItemFull);
        }

        for item in &self.items {
            item.validate()?;
        }

        if self.collections.len() > MAX_COLLECTIONS_PER_METADATA {
            return err!(MythicMetadataError::MetadataCollectionFull);
        }

        for collection in &self.collections {
            collection.validate()?;
        }

        Ok(())
    }
}

#[account]
pub struct Metadata {
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
    /// - Should it also be allowed to close the account?
    pub issuing_authority: Pubkey,

    /// A set of metadata collections
    pub collection: MetadataRootCollection,

    /// Bump
    pub bump: u8,
}

impl Metadata {
    pub fn size(root_collection: &MetadataRootCollection) -> usize {
        let root_collection_size =
            MetadataRootCollection::size(&root_collection.items, &root_collection.collections);

        8 + // Anchor discriminator
        32 + // subject
        32 + // issuing_authority
        1 + 32 + // update_authority
        root_collection_size + // root_collection
        1 // bump
    }

    pub fn validate(&self) -> Result<()> {
        self.collection.validate()?;
        Ok(())
    }
}
