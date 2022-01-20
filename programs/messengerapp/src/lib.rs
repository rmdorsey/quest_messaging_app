use anchor_lang::prelude::*;

//program id received after deployment
declare_id!("7F4u7AazmDPTtnm6BdeRj3urrovvorW9LEnLbkfP9tfm");

#[program]
pub mod messengerapp {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>, data: String) -> ProgramResult {
        //we would want our main account (the account that will 
        //handle all the messaging stuff of the program) to have 
        //two fields, one for storing the incoming data and the 
        //other for keeping a record of all earlier data.
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        //we will be adding to the data_list everytime a new data is introduced
        base_account.data_list.push(copy);
        //Ok(()) like a gate, that lets the program continue if 
        //there are no errors but sends the program into another 
        //state if an error is encountered
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }
}

//- Whenever we need to include multiple accounts in a struct, 
//we would use the derive Accounts macro
//- When we want to derive an account to pass to the 
//function using other accounts, we use the derive accounts 
//macro and while defining a singular account we would simply 
//use the normal account macro, which is [account]
//- we used the derive Accounts macro since we had to 
//incorporate 3 accounts here and for all these three accounts 
//individually, we used the account macro
#[derive(Accounts)]
pub struct Initialize<'info> {
    //- init macro is used to create a new account owned by the 
    //current program which is our messengerapp program. Whenever 
    //init parameter is used, we must always specify the payer 
    //or the account that will be paying for creation of the 
    //account on the Solana blockchain, along with the space 
    //param which is the space with which the new account is created.
    // - Space is referring to the two fields (data and data_list)
    #[account(init, payer = user, space = 64 + 64)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    //- Signer type: This is used to enforce the constraint that 
    //the authority account (messengerapp in this case) signed 
    //the transaction.
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub data: String,
    //- A vector is a list of elements with no specified size. But, bear in mind that initially 
    //we had createed our base account with the space of 64 + 64 so there will be a limit 
    //to how many messages can be stored.
    pub data_list: Vec<String>,
}