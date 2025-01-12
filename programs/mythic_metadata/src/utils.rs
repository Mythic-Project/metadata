use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::errors::*;
use crate::state::*;

pub fn check_collection_metadata_equality(
    metadata_collection_metadata_key: &Account<MetadataKey>,
    collection_metadata_key: &Account<MetadataKey>,
) -> bool {
    metadata_collection_metadata_key.key() == collection_metadata_key.key()
        && metadata_collection_metadata_key.id == collection_metadata_key.id
}

pub fn verify_metadata_update_authority(
    metadata: &Metadata,
    update_authority: &Pubkey,
) -> Result<bool> {
    if metadata.update_authority.is_none() {
        return err!(MythicMetadataError::ImmutableMetadata);
    }

    if let Some(expected_update_authority) = metadata.update_authority {
        if expected_update_authority.ne(&update_authority) {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn verify_collection_update_authority(
    metadata: &Metadata,
    collection_metadata_key_id: u64,
    update_authority: &Pubkey,
) -> Result<(usize, MetadataCollection)> {
    match metadata
        .collections
        .binary_search_by_key(&collection_metadata_key_id, |collection| {
            collection.metadata_key_id
        }) {
        Ok(collection_index) => {
            let collection = metadata.collections.get(collection_index).unwrap();

            if collection.update_authority.is_none() {
                require!(
                    verify_metadata_update_authority(metadata, update_authority)?,
                    MythicMetadataError::Unauthorized
                );
                return Ok((collection_index, collection.clone()));
            } else {
                let collection_update_authority = collection.update_authority.unwrap();
                if collection_update_authority.ne(&update_authority) {
                    require!(
                        verify_metadata_update_authority(metadata, update_authority)?,
                        MythicMetadataError::Unauthorized
                    );
                }
                return Ok((collection_index, collection.clone()));
            }
        }
        Err(_) => return err!(MythicMetadataError::MetadataCollectionNonExistent),
    };
}

pub fn realloc_account<'a>(
    account: AccountInfo<'a>,
    new_account_size: usize,
    rent_payer: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
) -> Result<()> {
    require_keys_eq!(
        *account.owner,
        crate::id(),
        MythicMetadataError::InvalidAccountOwner
    );

    let current_account_size = account.data.borrow().len();
    if current_account_size >= new_account_size {
        return Ok(());
    }

    let current_lamports = account.lamports();
    let rent_exempt_lamports = Rent::get()?.minimum_balance(new_account_size);

    let lamports_diff = rent_exempt_lamports.saturating_sub(current_lamports);
    if lamports_diff.gt(&0) {
        transfer(
            CpiContext::new(
                system_program,
                Transfer {
                    from: rent_payer,
                    to: account.clone(),
                },
            ),
            lamports_diff,
        )?;
    }

    AccountInfo::realloc(&account, new_account_size, false)?;
    Ok(())
}
