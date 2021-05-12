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

impl Price {
    pub fn get_empty() -> Self {
        Price {
            symbol: [0u8; 8],
            rate: 0,
            last_updated: 0,
            request_id: 0,
        }
    }

    pub fn get_empty_prices(size: u8) -> Vec<Price> {
        (0..size).map(|_| Price::get_empty()).collect()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct PriceDBKeeper {
    owner: [u8; 32],
    current_size: u8,
    prices: Vec<Price>,
}

impl PriceDBKeeper {
    pub fn get_empty(size: u8) -> Self {
        PriceDBKeeper {
            owner: [0; 32],
            current_size: 0,
            prices: Price::get_empty_prices(size),
        }
    }
}

/// Commands supported by the program
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum Command {
    // account 0: PriceDBKeeper account
    Init(u8, [u8; 32]),
    TransferOwnership([u8; 32]),
    Relay(Vec<Price>),
    Remove(Vec<[u8; 8]>),
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
        Command::Init(size, owner) => {
            msg!("Init!");
            let pdbk_account = next_account_info(account_info_iter)?;
            let temp = (*pdbk_account.data.borrow()).to_vec();
            if is_initialized(&temp) {
                Err(ProgramError::AccountAlreadyInitialized)
            } else {
                match PriceDBKeeper::try_from_slice(&temp) {
                    Ok(_) => Err(ProgramError::InvalidArgument),
                    Err(_) => {
                        let mut pdbk = PriceDBKeeper::get_empty(size);
                        pdbk.owner = owner;
                        pdbk.serialize(&mut &mut pdbk_account.data.borrow_mut()[..])?;
                        Ok(())
                    }
                }
            }
        }
        Command::TransferOwnership(new_owner) => {
            msg!("TransferOwnership!");
            let pdbk_account = next_account_info(account_info_iter)?;
            let sender = next_account_info(account_info_iter)?;
            if !sender.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            let temp = (*pdbk_account.data.borrow()).to_vec();
            if !is_initialized(&temp) {
                return Err(ProgramError::UninitializedAccount);
            }

            let mut pdbk = PriceDBKeeper::try_from_slice(&temp).map_err(|_| ProgramError::Custom(113))?;

            // check owner
            if pdbk.owner != sender.key.to_bytes() {
                return Err(ProgramError::Custom(112));
            }

            // set owner
            pdbk.owner = new_owner;
            // save state
            pdbk.serialize(&mut &mut pdbk_account.data.borrow_mut()[..])?;
            Ok(())
        }
        Command::Relay(prices) => {
            msg!("Relay!");
            let pdbk_account = next_account_info(account_info_iter)?;
            let sender = next_account_info(account_info_iter)?;
            if !sender.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            let temp = (*pdbk_account.data.borrow()).to_vec();
            if !is_initialized(&temp) {
                return Err(ProgramError::UninitializedAccount);
            }

            let mut pdbk =
                PriceDBKeeper::try_from_slice(&temp).map_err(|_| ProgramError::Custom(113))?;

            // check owner
            if pdbk.owner != sender.key.to_bytes() {
                return Err(ProgramError::Custom(112));
            }

            // create an array for new prices
            let mut new_prices: Vec<Price> = vec![];

            // replace or add the new one
            for price in prices {
                let mut replace = false;
                for current_price in pdbk.prices.iter_mut() {
                    if current_price.symbol == price.symbol {
                        current_price.rate = price.rate;
                        current_price.last_updated = price.last_updated;
                        current_price.request_id = price.request_id;

                        replace = true;
                        break;
                    } else if current_price.symbol == [0u8; 8] {
                        break;
                    }
                }
                if !replace {
                    new_prices.push(price)
                }
            }

            let new_size = (pdbk.current_size as usize)+new_prices.len();
            if new_size > pdbk.prices.len() {
                // reach maximum size
                return Err(ProgramError::Custom(114));
            }

            // append new prices
            for j in 0..(new_size - (pdbk.current_size as usize)) {
                if let Some(p) = pdbk.prices.get_mut(j + (pdbk.current_size as usize)) {
                    p.symbol = new_prices[j].symbol;
                    p.rate = new_prices[j].rate;
                    p.last_updated = new_prices[j].last_updated;
                    p.request_id = new_prices[j].request_id;
                }
            }

            // change current_size to new_size
            pdbk.current_size = new_size as u8;

            // save state
            pdbk.serialize(&mut &mut pdbk_account.data.borrow_mut()[..])?;
            Ok(())
        }
        Command::Remove(symbols) => {
            msg!("Remove!");
            let pdbk_account = next_account_info(account_info_iter)?;
            let sender = next_account_info(account_info_iter)?;
            if !sender.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            let temp = (*pdbk_account.data.borrow()).to_vec();
            if !is_initialized(&temp) {
                return Err(ProgramError::UninitializedAccount);
            }

            let mut pdbk =
                PriceDBKeeper::try_from_slice(&temp).map_err(|_| ProgramError::Custom(113))?;

            // check owner
            if pdbk.owner != sender.key.to_bytes() {
                return Err(ProgramError::Custom(112));
            }

            // remove every symbol in symbols
            let remain_prices: Vec<Price> = pdbk.prices.clone().into_iter().filter(
                |p| (p.symbol != [0u8; 8]) && symbols.iter().all(|&s| s != p.symbol)
            ).collect();

            // pad array at the end
            for (i, current_price) in pdbk.prices.iter_mut().enumerate() {
                if i < remain_prices.len() {
                    current_price.symbol = remain_prices[i].symbol;
                    current_price.rate = remain_prices[i].rate;
                    current_price.last_updated = remain_prices[i].last_updated;
                    current_price.request_id = remain_prices[i].request_id;
                } else {
                    current_price.symbol = [0u8; 8];
                    current_price.rate = 0;
                    current_price.last_updated = 0;
                    current_price.request_id = 0;
                }
            }

            // set current size
            pdbk.current_size = remain_prices.len() as u8;

            // save state
            pdbk.serialize(&mut &mut pdbk_account.data.borrow_mut()[..])?;
            Ok(())
        }
    }
}
