use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct AppendMetadataItem<'info> {
    #[account()]
    pub update_authority: Signer<'info>,
    #[account(
        mut,
        has_one = metadata_key @ DaoMetadataError::InvalidMetadataKeyField,
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
            metadata_key.namespace_authority.as_ref(),
            metadata_key.name.as_bytes()
        ],
        bump,
    )]
    pub metadata_key: Account<'info, MetadataKey>,
    #[account(
        seeds = [
            PREFIX,
            METADATA_KEY,
            collection_metadata_key.namespace_authority.as_ref(),
            collection_metadata_key.name.as_bytes()
        ],
        bump,
    )]
    pub collection_metadata_key: Account<'info, MetadataKey>,
    #[account(
        seeds = [
            PREFIX,
            METADATA_KEY,
            item_metadata_key.namespace_authority.as_ref(),
            item_metadata_key.name.as_bytes()
        ],
        bump,
    )]
    pub item_metadata_key: Account<'info, MetadataKey>,
}

pub fn handler(ctx: Context<AppendMetadataItem>, args: AppendMetadataItemArgs) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let collection_metadata_key = &ctx.accounts.collection_metadata_key;
    let item_metadata_key = &ctx.accounts.item_metadata_key;
    let update_authority = &ctx.accounts.update_authority;

    let (collection_index, mut collection) = verify_collection_update_authority(
        &metadata,
        &collection_metadata_key.key(),
        &update_authority.key(),
    )?;

    match collection
        .items
        .binary_search_by_key(&item_metadata_key.key(), |item| item.metadata_key)
    {
        Ok(_) => return err!(DaoMetadataError::MetadataItemAlreadyExists),
        Err(item_index) => {
            let slot = Clock::get()?.slot;
            collection.update_slot = slot;
            collection.items.insert(
                item_index,
                MetadataItem {
                    metadata_key: item_metadata_key.key(),
                    update_slot: slot,
                    value: args.value,
                },
            );
            metadata.collections.remove(collection_index);
            metadata.collections.insert(collection_index, collection);
        }
    };

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AppendMetadataItemArgs {
    pub value: Vec<u8>,
}
