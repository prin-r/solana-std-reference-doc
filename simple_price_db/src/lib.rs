use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct Price {
    symbol: [u8; 8],
    rate: u64,
    last_updated: u64,
    request_id: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct StdReferenceBasic {
    owner: [u8; 32],
    current_size: u8,
    prices: Vec<Price>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct SimplePriceDB {
    owner: [u8; 32],
    latest_symbol: [u8; 8],
    latest_price: u64,
}

/// Commands supported by the program
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum Command {
    Init([u8; 32]),
    TransferOwnership([u8; 32]),
    SetPrice([u8; 8]),
}

fn is_initialized(arr: &Vec<u8>) -> bool {
    arr.iter().fold(0u32, |s, &x| s + (x as u32)) > 0
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
fn process_instruction<'a>(
    _program_id: &Pubkey, // Public key of the account the pricedb program was loaded into
    accounts: &'a [AccountInfo<'a>], // The accounts to be interacted with
    instruction_data: &[u8], // borsh encoded of Command
) -> ProgramResult {
    msg!("Begin pricedb Rust program entrypoint");

    let command = Command::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let account_info_iter = &mut accounts.iter();

    match command {
        Command::Init(owner) => {
            msg!("Init!");
            let proxy_account = next_account_info(account_info_iter)?;
            let temp = (*proxy_account.data.borrow()).to_vec();

            // error if the contract is initialized
            if is_initialized(&temp) {
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            // convert raw account's data in to SimplePriceDB struct
            let mut spdb = SimplePriceDB::try_from_slice(&temp).map_err(|_| ProgramError::Custom(113))?;

            // set owner
            spdb.owner = owner;

            // set latest_symbol and latest_price to zero values
            spdb.latest_symbol = [0u8; 8];
            spdb.latest_price = 0u64;

            // save change
            spdb.serialize(&mut &mut proxy_account.data.borrow_mut()[..])?;

            Ok(())
        }
        Command::TransferOwnership(new_owner) => {
            msg!("TransferOwnership!");
            let simple_price_db = next_account_info(account_info_iter)?;
            let sender = next_account_info(account_info_iter)?;
            if !sender.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            let temp = (*simple_price_db.data.borrow()).to_vec();

            // error if not initialized
            if !is_initialized(&temp) {
                return Err(ProgramError::UninitializedAccount);
            }

            // parse raw account's data into SimplePriceDB struct
            let mut spdb = SimplePriceDB::try_from_slice(&temp).map_err(|_| ProgramError::Custom(113))?;

            // check owner
            if spdb.owner != sender.key.to_bytes() {
                return Err(ProgramError::Custom(112));
            }

            // set owner
            spdb.owner = new_owner;

            // save change
            spdb.serialize(&mut &mut simple_price_db.data.borrow_mut()[..])?;
            Ok(())
        }
        Command::SetPrice(symbol) => {
            msg!("SetPrice!");
            let simple_price_db = next_account_info(account_info_iter)?;
            let sender = next_account_info(account_info_iter)?;
            let std_reference_account = next_account_info(account_info_iter)?;

            // check that sender is signer
            if !sender.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            let temp = (*simple_price_db.data.borrow()).to_vec();

            // error if not initialized
            if !is_initialized(&temp) {
                return Err(ProgramError::UninitializedAccount);
            }

            // parse raw account's data into SimplePriceDB struct
            let mut spdb = SimplePriceDB::try_from_slice(&temp).map_err(|_| ProgramError::Custom(113))?;

            // check owner
            if spdb.owner != sender.key.to_bytes() {
                return Err(ProgramError::Custom(112));
            }

            // get std_reference's state
            let temp2 = (*std_reference_account.data.borrow()).to_vec();
            let std_reference = StdReferenceBasic::try_from_slice(&temp2).map_err(|_| ProgramError::Custom(113))?;

            // filter only a price of the symbol
            let rate = std_reference.prices.iter().find(|&p| p.symbol == symbol).map_or(None, |p| Some(p.rate));
            if rate.is_none() {
                msg!("Price not found !");
                return Err(ProgramError::Custom(115));
            }

            // set latest price and symbol
            spdb.latest_price = rate.unwrap();
            spdb.latest_symbol = symbol;

            // save change
            spdb.serialize(&mut &mut simple_price_db.data.borrow_mut()[..])?;
            Ok(())
        }
    }
}
