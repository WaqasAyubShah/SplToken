use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, TokenAccount, Mint}, associated_token::AssociatedToken};
use mpl_token_metadata::state::DataV2;

declare_id!(""); ///put the program id here

#[program]
pub mod anchor_spl_token {
    use anchor_lang::system_program;
    use anchor_spl::{token::{initialize_mint, InitializeMint, mint_to, MintTo, transfer, Transfer, burn, Burn, freeze_account, FreezeAccount, close_account, CloseAccount, thaw_account, ThawAccount, set_authority, SetAuthority, spl_token::instruction::AuthorityType}, associated_token, metadata::{create_metadata_accounts_v3, create_master_edition_v3}};
    // use mpl_token_metadata::instruction::CreateMasterEdition;
    

    use super::*;

    pub fn create_token(ctx: Context<CreateToken>,decimals:u8,amount:u64) -> Result<()> {

        system_program::create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(), 
                system_program::CreateAccount { from: ctx.accounts.signer.to_account_info(), to: ctx.accounts.mint_token.to_account_info() }
            ), 
            10_000_000, 
            82, 
            ctx.accounts.token_program.key
        )?;

        initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InitializeMint{mint:ctx.accounts.mint_token.to_account_info(),rent:ctx.accounts.rent.to_account_info()}
            ), 
            decimals, 
            ctx.accounts.signer.key, 
            Some(ctx.accounts.signer.key)
        )?;


        associated_token::create(
            CpiContext::new(
                ctx.accounts.associate_token_program.to_account_info(), 
                associated_token::Create { 
                    payer: ctx.accounts.signer.to_account_info(), 
                    associated_token: ctx.accounts.token_account.to_account_info(), 
                    authority: ctx.accounts.signer.to_account_info(), 
                    mint: ctx.accounts.mint_token.to_account_info(), 
                    system_program: ctx.accounts.system_program.to_account_info(), 
                    token_program: ctx.accounts.token_program.to_account_info() 
                }
            )
        )?;

        mint_to(
            CpiContext::new(
                ctx.accounts.token_account.to_account_info(), 
                MintTo{authority:ctx.accounts.signer.to_account_info(),mint:ctx.accounts.mint_token.to_account_info(),to:ctx.accounts.token_account.to_account_info()}
            ), 
            amount
        )?;

        Ok(())
    }

    pub fn transer_token(ctx: Context<TransferToken>,amount:u64)->Result<()>{

        msg!("Started {:} tokens transfer from account {:} to {:}",amount,ctx.accounts.from_account.key(),ctx.accounts.to_account.key());

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                Transfer{authority:ctx.accounts.signer.to_account_info(),from:ctx.accounts.from_account.to_account_info(),to:ctx.accounts.to_account.to_account_info()}
            ), 
            amount
        )?;

        Ok(())
    }

    pub fn set_authority_token(ctx: Context<SetAuthorityToken>,authority_value:u8)->Result<()>{
        let account_or_mint;
        let authority_type;
        match authority_value {
            0=> {
                authority_type = anchor_spl::token::spl_token::instruction::AuthorityType::MintTokens;
                account_or_mint=ctx.accounts.mint_token.to_account_info();
            },
            1=> {
                authority_type = anchor_spl::token::spl_token::instruction::AuthorityType::FreezeAccount;
                account_or_mint=ctx.accounts.mint_token.to_account_info();
            },
            2 => {
                authority_type = anchor_spl::token::spl_token::instruction::AuthorityType::AccountOwner;
                account_or_mint = ctx.accounts.token_account.to_account_info();
            },
            _ => {
                authority_type = anchor_spl::token::spl_token::instruction::AuthorityType::CloseAccount;
                account_or_mint = ctx.accounts.token_account.to_account_info();
            }
        }
        set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                SetAuthority{
                    account_or_mint:account_or_mint,
                    current_authority:ctx.accounts.signer.to_account_info()
                }
            ), 
            authority_type.clone(), 
            Some(ctx.accounts.new_signer.key())
        )?;

        Ok(())
    }

    pub fn burn_token(ctx: Context<BurnToken>,amount:u64)->Result<()>{
        burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                Burn{
                    authority:ctx.accounts.signer.to_account_info(),
                    from:ctx.accounts.token_account.to_account_info(),
                    mint:ctx.accounts.mint_token.to_account_info()
                }
            ), 
            amount
        )?;
        Ok(())
    }

    pub fn freeze_token(ctx: Context<FreezeToken>)->Result<()>{
        
        freeze_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                FreezeAccount{
                    account:ctx.accounts.token_account.to_account_info(),
                    mint:ctx.accounts.mint_token.to_account_info(),
                    authority:ctx.accounts.signer.to_account_info(),
                }
            )
        )?;


        Ok(())
    }

    pub fn un_freeze_token(ctx: Context<FreezeToken>)->Result<()>{
        
        thaw_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                ThawAccount{
                    account:ctx.accounts.token_account.to_account_info(),
                    mint:ctx.accounts.mint_token.to_account_info(),
                    authority:ctx.accounts.signer.to_account_info(),
                }
            )
        )?;


        Ok(())
    }

    pub fn close_token(ctx: Context<CloseToken>)->Result<()>{
        
        close_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                CloseAccount{
                    account:ctx.accounts.token_account.to_account_info(),
                    destination:ctx.accounts.signer.to_account_info(),
                    authority:ctx.accounts.signer.to_account_info(),
                }
            )
        )?;


        Ok(())
    }

    pub fn set_token_metadata(ctx: Context<CreateMetadata>, data:MetadataData)->Result<()>{
        let (metadata_address,b1) = Pubkey::find_program_address(&[
            b"metadata", 
            &ctx.accounts.metadata_program.key.to_bytes(),
            &ctx.accounts.mint_token.key().to_bytes()
            ], 
            ctx.accounts.metadata_program.key
        );

        let metadata_account = &ctx.accounts.metadata_account;
        let master_account = &ctx.accounts.master_account;

        if metadata_address != *metadata_account.key{
            return err!(ProgramErrors::PdaNotMatched)
        }
        
        let (master_address,b2) = Pubkey::find_program_address(&[
            b"metadata", 
            &ctx.accounts.metadata_program.key.to_bytes(),
            &ctx.accounts.mint_token.key().to_bytes(),
            b"edition"
            ], 
            ctx.accounts.metadata_program.key
        );

        if master_address != *master_account.key{
            return err!(ProgramErrors::PdaNotMatched)
        }

        create_metadata_accounts_v3(
            CpiContext::new(
                ctx.accounts.metadata_program.to_account_info(), 
                anchor_spl::metadata::CreateMetadataAccountsV3{
                    metadata:metadata_account.to_account_info(),
                    mint:ctx.accounts.mint_token.to_account_info(),
                    mint_authority:ctx.accounts.signer.to_account_info(),
                    update_authority:ctx.accounts.signer.to_account_info(),
                    payer:ctx.accounts.signer.to_account_info(),
                    system_program:ctx.accounts.system_program.to_account_info(),
                    rent:ctx.accounts.rent.to_account_info(),
                }
            ), 
            DataV2 { 
                name: data.name, 
                symbol: data.symbol, 
                uri: data.uri, 
                seller_fee_basis_points: data.seller_fee_basis_points,
                creators: None, 
                collection: None, 
                uses: None 
            },
            true, 
            true, 
            None
        )?;


        create_master_edition_v3(
            CpiContext::new(
                ctx.accounts.metadata_program.to_account_info(), 
                anchor_spl::metadata::CreateMasterEditionV3 {
                    metadata:metadata_account.to_account_info(),
                    mint:ctx.accounts.mint_token.to_account_info(),
                    mint_authority:ctx.accounts.signer.to_account_info(),
                    update_authority:ctx.accounts.signer.to_account_info(),
                    payer:ctx.accounts.signer.to_account_info(),
                    system_program:ctx.accounts.system_program.to_account_info(),
                    rent:ctx.accounts.rent.to_account_info(),
                    edition:ctx.accounts.edition_account.to_account_info(),
                    token_program:ctx.accounts.token_program.to_account_info()
                }
            ), 
            Some(data.suply)
        )?;

        Ok(())
    }
}

#[derive(Debug,AnchorDeserialize,AnchorSerialize)]
pub struct MetadataData{
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub suply: u64
    // pub creators: Option<Vec<Creator>>,
    // pub collection: Option<Collection>,
    // pub uses: Option<Uses>,
}

#[derive(Accounts)]
pub struct CreateMetadata<'info> {
    #[account(mut)]
    pub mint_token:Account<'info,Mint>,
    #[account(mut)]
    pub signer:Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub metadata_account:AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub master_account:AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub edition_account:AccountInfo<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associate_token_program:Program<'info,AssociatedToken>,
    /// CHECK:
    pub metadata_program:AccountInfo<'info>,
    pub rent:Sysvar<'info,Rent>
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub mint_token:Signer<'info>,
    #[account(mut)]
    pub signer:Signer<'info>,
    ///CHECK:
    #[account(mut)]
    pub token_account:AccountInfo<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associate_token_program:Program<'info,AssociatedToken>,
    pub rent:Sysvar<'info,Rent>
}

#[derive(Accounts)]
pub struct TransferToken<'info>{    
    #[account(mut)]
    pub mint_token:Account<'info,Mint>,
    #[account(mut)]
    pub from_account:Account<'info,TokenAccount>,
    #[account(mut)]
    pub to_account:Account<'info,TokenAccount>,
    #[account(mut)]
    pub signer:Signer<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associate_token_program:Program<'info,AssociatedToken>,
}

#[derive(Accounts)]
pub struct SetAuthorityToken<'info> {
    #[account(mut)]
    pub mint_token:Account<'info,Mint>,
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(mut)]
    pub new_signer:Signer<'info>,
    #[account(mut)]
    pub token_account:Account<'info,TokenAccount>,
    pub token_program:Program<'info,Token>,
}

#[derive(Accounts)]
pub struct BurnToken<'info> {
    #[account(mut)]
    pub mint_token:Account<'info,Mint>,
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(mut)]
    pub token_account:Account<'info,TokenAccount>,
    pub token_program:Program<'info,Token>,
}

#[derive(Accounts)]
pub struct FreezeToken<'info> {
    #[account(mut)]
    pub mint_token:Account<'info,Mint>,
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(mut)]
    pub token_account:Account<'info,TokenAccount>,
    pub token_program:Program<'info,Token>,
}

#[derive(Accounts)]
pub struct CloseToken<'info> {
    #[account(mut)]
    pub mint_token:Account<'info,Mint>,
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(mut)]
    pub token_account:Account<'info,TokenAccount>,
    pub token_program:Program<'info,Token>,
}

#[error_code]
pub enum ProgramErrors {
    #[msg("PDA account not matched")]
    PdaNotMatched
}
