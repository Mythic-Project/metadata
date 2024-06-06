use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct RemoveMetadataItem<'info> {
    #[account()]
    pub update_authority: Signer<'info>,
    #[account(
        mut,
        constraint = metadata.collection.metadata_key_id.eq(&root_collection_metadata_key.id) @ MythicMetadataError::InvalidMetadataKey,
        seeds = [
            PREFIX,
            METADATA,
            root_collection_metadata_key.key().as_ref(),
            metadata.issuing_authority.as_ref(),
            metadata.subject.as_ref()
        ],
        bump
    )]
    pub metadata: Account<'info, Metadata>,
    #[account(
        seeds = [
            PREFIX,
            METADATA_KEY,
            &root_collection_metadata_key.id.to_le_bytes()
        ],
        bump,
    )]
    pub root_collection_metadata_key: Account<'info, MetadataKey>,
    #[account(
        seeds = [
            PREFIX,
            METADATA_KEY,
            &collection_metadata_key.id.to_le_bytes()
        ],
        bump,
    )]
    pub collection_metadata_key: Account<'info, MetadataKey>,
    #[account(
        seeds = [
            PREFIX,
            METADATA_KEY,
            &item_metadata_key.id.to_le_bytes()
        ],
        bump,
    )]
    pub item_metadata_key: Account<'info, MetadataKey>,
}

pub fn handler(ctx: Context<RemoveMetadataItem>) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let root_collection_metadata_key = &ctx.accounts.root_collection_metadata_key;
    let collection_metadata_key = &ctx.accounts.collection_metadata_key;
    let item_metadata_key = &ctx.accounts.item_metadata_key;
    let update_authority = &ctx.accounts.update_authority;

    // Check if metadata item is to be removed in root collection
    if check_collection_root_collection_equality(
        root_collection_metadata_key,
        collection_metadata_key,
    ) {
        verify_root_collection_update_authority(&metadata.collection, &update_authority.key())?;

        match metadata
            .collection
            .items
            .binary_search_by_key(&item_metadata_key.id, |item| item.metadata_key_id)
        {
            Ok(item_index) => metadata.collection.items.remove(item_index),
            Err(_) => return err!(MythicMetadataError::MetadataItemNonExistent),
        };
    } else {
        let (collection_index, mut collection) = verify_collection_update_authority(
            &metadata.collection,
            collection_metadata_key.id,
            &update_authority.key(),
        )?;

        match collection
            .items
            .binary_search_by_key(&item_metadata_key.id, |item| item.metadata_key_id)
        {
            Ok(item_index) => {
                collection.items.remove(item_index);
                metadata.collection.collections.remove(collection_index);
                metadata
                    .collection
                    .collections
                    .insert(collection_index, collection);
            }
            Err(_) => return err!(MythicMetadataError::MetadataItemNonExistent),
        };
    }

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RemoveMetadataItemArgs {
    pub value: Vec<u8>,
}
