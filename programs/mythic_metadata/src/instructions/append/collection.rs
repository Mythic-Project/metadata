use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct AppendMetadataCollection<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
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
        has_one = issuing_authority,
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
    pub system_program: Program<'info, System>,
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

    metadata.validate()?;
    let new_account_size = Metadata::size(&metadata.items, &metadata.collections);
    realloc_account(
        ctx.accounts.metadata.to_account_info(),
        new_account_size,
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AppendMetadataCollectionArgs {
    pub update_authority: Option<Pubkey>,
}
