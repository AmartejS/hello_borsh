use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use serde_json::{Result,Value};
use std::str::FromStr;
//use serde::{Deserialize,Serialize};

//use std::str;
mod instruction;
use crate::instruction::HelloInstruction;
use core::convert::From;


/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
   
    pub name: u32,
    pub age:  u32,
    
}
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Instructiondata {
    
    address: String,
    
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);


// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
  msg!("Rust program entrypoint");
 
  msg!("instruction data: {:?}", _instruction_data);
   

  let message = Instructiondata::try_from_slice(_instruction_data).unwrap();
    
  let accounts_iter = &mut accounts.iter();
    
    // Get msg!the account to say hello to
    let account = next_account_info(accounts_iter)?;
    
    msg!("Instruction data message ==> {:?}", message);
   // msg!("user address : {:?}", useraddress);
   for i in 0..accounts.len() 
   {
    let useraddress = Pubkey::to_string(accounts[i].key);

   
     if useraddress == message.address
     {
        msg!("User key matched");
        let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
        msg!("Account info {:?}", greeting_account);
       // greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
     }
     else
     {
      msg!("No account with address {}  found",message.address);

     }
   } 
  

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
 }


    Ok(())
  }


 