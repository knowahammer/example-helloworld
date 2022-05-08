use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type, but where is it's unexecutable account?
/// Use the serialize macro to get the struct from the binary data
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ThumbState {
    pub thumbs: u32,
}

// Declare the entry function name by this macro
entrypoint!(process_instruction);

// Implementation of the entry function
pub fn process_instruction(
    program_id: &Pubkey, // The public key generated automatically after the deployment?
    accounts: &[AccountInfo], // The account to say hello to? Not the owner?
    _instruction_data: &[u8], // Ignored, not used in this hello world example
) -> ProgramResult {
    // validate the account
    let mut iter = accounts.iter();
    let next = iter.next();
    if let None = next{
        return Err(ProgramError::InvalidAccountData);
    }
    let account = next.unwrap();

    if account.owner != program_id{
        return Err(ProgramError::IncorrectProgramId);
    }

    // deserialize the data
    let thumbs_result = ThumbState::try_from_slice(&account.data.borrow());
    if let Err(err) = thumbs_result{
        return Err(ProgramError::BorshIoError(err.to_string()));
    }
    let mut thumb = thumbs_result.unwrap();

    // change the state
    thumb.thumbs += 1;

    // just serialize the data, and with nothing else?
    let mut m = (*account.data).borrow_mut();
    thumb.serialize(&mut &mut m[..])?;

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            ThumbState::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .thumbs,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            ThumbState::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .thumbs,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            ThumbState::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .thumbs,
            2
        );

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        println!("{:?}",  ThumbState::try_from_slice(&accounts[0].data.borrow()).unwrap().thumbs); // print 3
    }
}
