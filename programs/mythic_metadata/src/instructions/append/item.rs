use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct AppendMetadataItem<'info> {
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
        bump = metadata.bump,
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

pub fn handler(ctx: Context<AppendMetadataItem>, args: AppendMetadataItemArgs) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let metadata_metadata_key = &ctx.accounts.metadata_metadata_key;
    let collection_metadata_key = &ctx.accounts.collection_metadata_key;
    let item_metadata_key = &ctx.accounts.item_metadata_key;

    // Check if metadata item is to be appended in main metadata
    if check_collection_metadata_equality(metadata_metadata_key, collection_metadata_key) {
        match metadata
            .items
            .binary_search_by_key(&item_metadata_key.id, |item| item.metadata_key_id)
        {
            Ok(_) => return err!(MythicMetadataError::MetadataItemAlreadyExists),
            Err(item_index) => {
                let slot = Clock::get()?.slot;
                metadata.update_slot = slot;
                metadata.items.insert(
                    item_index,
                    MetadataItem {
                        metadata_key_id: item_metadata_key.id,
                        update_slot: slot,
                        value: args.value,
                    },
                );
            }
        };
    } else {
        match metadata
            .collections
            .binary_search_by_key(&collection_metadata_key.id, |collection| {
                collection.metadata_key_id
            }) {
            Ok(collection_index) => {
                let mut collection = metadata.collections.get_mut(collection_index).unwrap().clone();
                
                match collection
                    .items
                    .binary_search_by_key(&item_metadata_key.id, |item| item.metadata_key_id)
                    {
                        Ok(_) => return err!(MythicMetadataError::MetadataItemAlreadyExists),
                        Err(item_index) => {
                            let slot = Clock::get()?.slot;
                            collection.update_slot = slot;
                            collection.items.insert(
                                item_index,
                                MetadataItem {
                                    metadata_key_id: item_metadata_key.id,
                                    update_slot: slot,
                                    value: args.value,
                                },
                            );
                            metadata.collections.remove(collection_index);
                            metadata.collections.insert(collection_index, collection);
                        }
                    }
            }
            Err(_) => return err!(MythicMetadataError::MetadataCollectionNonExistent)
        };
    }

    let metadata_new_size = Metadata::size(&metadata.items, &metadata.collections);
    realloc_account(
        metadata.to_account_info(),
        metadata_new_size,
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;

    metadata.validate()?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AppendMetadataItemArgs {
    pub value: Vec<u8>,
}
