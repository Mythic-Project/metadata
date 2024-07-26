use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct UpdateMetadataItem<'info> {
    #[account(mut)]
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
    #[account(
        seeds = [
            PREFIX,
            METADATA_KEY,
            &item_metadata_key.id.to_le_bytes()
        ],
        bump = item_metadata_key.bump,
    )]
    pub item_metadata_key: Account<'info, MetadataKey>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateMetadataItem>, args: UpdateMetadataItemArgs) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let metadata_metadata_key = &ctx.accounts.metadata_metadata_key;
    let collection_metadata_key = &ctx.accounts.collection_metadata_key;
    let item_metadata_key = &ctx.accounts.item_metadata_key;
    let update_authority = &ctx.accounts.update_authority;

    // Check if metadata item is to be updated in main metadata
    if check_collection_metadata_equality(metadata_metadata_key, collection_metadata_key) {
        verify_metadata_update_authority(&metadata, &update_authority.key())?;

        match metadata
            .items
            .binary_search_by_key(&item_metadata_key.id, |item| item.metadata_key_id)
        {
            Ok(item_index) => {
                let slot = Clock::get()?.slot;
                metadata.items.remove(item_index);
                metadata.update_slot = slot;
                metadata.items.insert(
                    item_index,
                    MetadataItem {
                        metadata_key_id: item_metadata_key.id,
                        update_slot: slot,
                        value: args.new_value,
                    },
                );
            }
            Err(_) => return err!(MythicMetadataError::MetadataItemNonExistent),
        };
    } else {
        let (collection_index, mut collection) = verify_collection_update_authority(
            &metadata,
            collection_metadata_key.id,
            &update_authority.key(),
        )?;

        match collection
            .items
            .binary_search_by_key(&item_metadata_key.id, |item| item.metadata_key_id)
        {
            Ok(item_index) => {
                let slot = Clock::get()?.slot;
                collection.update_slot = slot;
                collection.items.remove(item_index);
                collection.items.insert(
                    item_index,
                    MetadataItem {
                        metadata_key_id: item_metadata_key.id,
                        update_slot: slot,
                        value: args.new_value,
                    },
                );
                metadata.collections.remove(collection_index);
                metadata.collections.insert(collection_index, collection);
            }
            Err(_) => return err!(MythicMetadataError::MetadataItemNonExistent),
        };
    }

    let metadata_new_size = Metadata::size(&metadata.items, &metadata.collections);
    realloc_account(
        metadata.to_account_info(),
        metadata_new_size,
        ctx.accounts.update_authority.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;

    metadata.validate()?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateMetadataItemArgs {
    pub new_value: Vec<u8>,
}
