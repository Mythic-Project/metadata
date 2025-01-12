use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct RevokeCollectionUpdateAuthority<'info> {
    pub issuing_authority: Signer<'info>,
    #[account(
        mut,
        constraint = metadata.metadata_key_id.eq(&metadata_metadata_key.id) @ MythicMetadataError::InvalidMetadataKey,
        seeds = [
            PREFIX,
            METADATA,
            metadata_metadata_key.key().as_ref(),
            issuing_authority.key().as_ref(),
            metadata.subject.as_ref()
        ],
        bump,
        has_one = issuing_authority
    )]
    pub metadata: Account<'info, Metadata>,
    #[account(
        seeds = [
            PREFIX,
            METADATA_KEY,
            &metadata_metadata_key.id.to_le_bytes()
        ],
        bump = metadata_metadata_key.bump,
    )]
    pub metadata_metadata_key: Account<'info, MetadataKey>,
    #[account(
        seeds = [
            PREFIX,
            METADATA_KEY,
            &collection_metadata_key.id.to_le_bytes()
        ],
        bump = collection_metadata_key.bump,
    )]
    pub collection_metadata_key: Account<'info, MetadataKey>,
}

pub fn handler(ctx: Context<RevokeCollectionUpdateAuthority>) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let metadata_metadata_key = &ctx.accounts.metadata_metadata_key;
    let collection_metadata_key = &ctx.accounts.collection_metadata_key;

    // Check if root collection and collection is same to update root_collection.update_authority
    if check_collection_metadata_equality(metadata_metadata_key, collection_metadata_key) {
        metadata.update_authority = None;
    } else {
        match metadata
            .collections
            .binary_search_by_key(&collection_metadata_key.id, |collection| {
                collection.metadata_key_id
            }) {
            Ok(collection_index) => {
                let mut collection = metadata.collections.get_mut(collection_index).unwrap().clone();
                collection.update_authority = None;
                metadata.collections.remove(collection_index);
                metadata.collections.insert(collection_index, collection);
            }
            Err(_) => return err!(MythicMetadataError::MetadataCollectionNonExistent)
        }
    }

    metadata.validate()?;

    Ok(())
}
