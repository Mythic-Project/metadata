use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct RemoveMetadataCollection<'info> {
    #[account()]
    pub update_authority: Signer<'info>,
    #[account(
        mut,
        constraint = metadata.metadata_key_id.eq(&metadata_metadata_key.id) @ MythicMetadataError::InvalidMetadataKey,
        seeds = [
            PREFIX,
            METADATA,
            metadata_metadata_key.key().as_ref(),
            metadata.issuing_authority.as_ref(),
            metadata.subject.as_ref()
        ],
        bump = metadata.bump,
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

pub fn handler(ctx: Context<RemoveMetadataCollection>) -> Result<()> {
    let metadata = &ctx.accounts.metadata;
    let update_authority = ctx.accounts.update_authority.key;

    // Verify metadata root collection update authority
    require!(
        verify_metadata_update_authority(&metadata, update_authority)?,
        MythicMetadataError::Unauthorized
    );

    let metadata = &mut ctx.accounts.metadata;
    let collection_metadata_key = &ctx.accounts.collection_metadata_key;

    match metadata
        .collections
        .binary_search_by_key(&collection_metadata_key.id, |collection| {
            collection.metadata_key_id
        }) {
        Ok(collection_index) => metadata.collections.remove(collection_index),
        Err(_) => return err!(MythicMetadataError::MetadataCollectionNonExistent),
    };

    Ok(())
}
