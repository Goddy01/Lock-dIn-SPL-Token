use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata as Metaplex,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
declare_id!("6QFJhtyFrtYHVFbzDat9k6UshybzKih5d8rksRvbvQea");

#[program]
pub mod td3_spl_token {
    use super::*;

    pub fn initiate_token(_ctx: Context<InitToken>, metadata: InitTokenParams) -> Result<()> {
        let seeds = &["mint".as_bytes(), &[_ctx.bumps.mint]];
        let signer = [&seeds[..]];

        let token_data: DataV2 = DataV2 {
            name: metadata.name,
            symbol: metadata.symbol,
            uri: metadata.uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let metadata_ctx = CpiContext::new_with_signer(
            _ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                payer: _ctx.accounts.payer.to_account_info(),
                update_authority: _ctx.accounts.mint.to_account_info(),
                mint: _ctx.accounts.mint.to_account_info(),
                metadata: _ctx.accounts.metadata.to_account_info(),
                mint_authority: _ctx.accounts.mint.to_account_info(),
                system_program: _ctx.accounts.system_program.to_account_info(),
                rent: _ctx.accounts.rent.to_account_info(),
            },
            &signer
            );

            create_metadata_accounts_v3(metadata_ctx, token_data, false, true, None)?;

            msg!("Token mint created successfully!");

            Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

 /// Instruction to initialize a new token mint and its metadata.
///
/// The `#[instruction(params: InitTokenParams)]` directive ensures
/// that the `params` are passed alongside the instruction call.
#[derive(Accounts)]
#[instruction(params: InitTokenParams)]
pub struct InitToken<'info> {
    /// The metadata account for the token.
    /// Using `UncheckedAccount` because metadata management is often handled
    /// by the Metaplex Token Metadata Program, which may not enforce Anchor constraints.
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// The mint account being created for the token.
    /// This account is derived programmatically using a PDA (Program Derived Address).
    #[account(
        init, // This attribute initializes the mint account.
        seeds = [b"mint"], // Seed to derive the PDA for the mint account.
        bump, // Auto-calculates the bump seed for PDA derivation.
        payer = payer, // Specifies the payer account funding the creation.
        mint::decimals = params.decimals, // Sets the token's precision (number of decimals).
        mint::authority = payer.key(), // Specifies the authority of the mint (payer in this case).
    )]
    pub mint: Account<'info, Mint>,

    /// The account paying for the transaction fees and account creation.
    /// This must be mutable because SOL will be debited from this account.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Rent sysvar account to ensure the new accounts are rent-exempt.
    pub rent: Sysvar<'info, Rent>,

    /// Solana System Program for account creation and other low-level operations.
    pub system_program: Program<'info, System>,

    /// Token Program, used to initialize and manage token-related operations.
    pub token_program: Program<'info, Token>,

    /// Metaplex Token Metadata Program for managing token metadata.
    pub token_metadata_program: Program<'info, Metaplex>,
}

/// Parameters for the `InitToken` instruction.
///
/// These parameters are passed by the client during the transaction.
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitTokenParams {
    /// The name of the token (e.g., "MyToken").
    pub name: String,

    /// The symbol of the token (e.g., "MTK").
    pub symbol: String,

    /// URI pointing to the token's metadata (e.g., JSON file with token details).
    pub uri: String,

    /// The number of decimal places for the token.
    pub decimals: u8,
}