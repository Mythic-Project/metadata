use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::errors::*;
use crate::state::*;

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
            let metadata_update_authority = metadata.update_authority;
            let collection = metadata.collections.get(collection_index).unwrap();

            let expected_collection_update_authority =
                if let Some(update_authority) = collection.update_authority {
                    update_authority
                } else {
                    metadata_update_authority
                };

            require_eq!(
                &expected_collection_update_authority,
                update_authority,
                MythicMetadataError::Unauthorized
            );

            return Ok((collection_index, collection.clone()));
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

    let lmaports_diff = rent_exempt_lamports.saturating_sub(current_lamports);
    if lmaports_diff.gt(&0) {
        transfer(
            CpiContext::new(
                system_program,
                Transfer {
                    from: rent_payer,
                    to: account.clone(),
                },
            ),
            lmaports_diff,
        )?;
    }

    AccountInfo::realloc(&account, new_account_size, false)?;
    Ok(())
}
