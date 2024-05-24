use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct AppendMetadataCollection<'info> {
    #[account()]
    pub update_authority: Signer<'info>,
    #[account(
        mut,
        has_one = update_authority @ MythicMetadataError::Unauthorized,
        constraint = metadata.metadata_key_id.eq(&metadata_key.id) @ MythicMetadataError::InvalidMetadataKey,
        seeds = [
            PREFIX,
            METADATA,
            metadata_key.key().as_ref(),
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
            &metadata_key.id.to_le_bytes()
        ],
        bump,
    )]
    pub metadata_key: Account<'info, MetadataKey>,
    #[account(
        seeds = [
            PREFIX,
            METADATA_KEY,
            &collection_metadata_key.id.to_le_bytes()
        ],
        bump,
    )]
    pub collection_metadata_key: Account<'info, MetadataKey>,
}

pub fn handler(
    ctx: Context<AppendMetadataCollection>,
    args: AppendMetadataCollectionArgs,
) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let collection_metadata_key = &ctx.accounts.collection_metadata_key;

    match metadata
        .collections
        .binary_search_by_key(&collection_metadata_key.id, |collection| {
            collection.metadata_key_id
        }) {
        Ok(_) => return err!(MythicMetadataError::MetadataCollectionAlreadyExists),
        Err(collection_index) => metadata.collections.insert(
            collection_index,
            MetadataCollection {
                metadata_key_id: collection_metadata_key.id,
                update_authority: args.update_authority,
                update_slot: Clock::get()?.slot,
                items: vec![],
            },
        ),
    };

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AppendMetadataCollectionArgs {
    pub update_authority: Option<Pubkey>,
}
