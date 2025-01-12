use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;
use crate::ID;

#[derive(Accounts)]
pub struct AppendMetadataItems<'info> {
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
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AppendMetadataItems>, args: AppendMetadataItemsArgs) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let metadata_metadata_key = &ctx.accounts.metadata_metadata_key;
    let collection_metadata_key = &ctx.accounts.collection_metadata_key;

    let mut index = 0;
    for item_account_info in ctx.remaining_accounts {
        if item_account_info.owner != &ID {
            return Err(anchor_lang::error!(
                anchor_lang::error::ErrorCode::AccountOwnedByWrongProgram
            )
            .with_account_name("MetadataKey"));
        }

        let item_buf = &mut &**item_account_info.try_borrow_mut_data()?;
        let item_metadata_key = MetadataKey::try_deserialize(item_buf)?;

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
                            value: args.value[index].clone(),
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
                                        value: args.value[index].clone(),
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
        index+=1;
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AppendMetadataItemsArgs {
    pub value: Vec<Vec<u8>>,
}
