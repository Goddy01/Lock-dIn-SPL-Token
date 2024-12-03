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

    /// The function to initialize the token mint and its metadata.
    ///
    /// This function performs all necessary steps to create a token and
    /// register its metadata with the Metaplex Token Metadata Program.
    pub fn initiate_token(_ctx: Context<InitToken>, metadata: InitTokenParams) -> Result<()> {
        // PDA seeds for the mint account
        let seeds = &["mint".as_bytes(), &[_ctx.bumps.mint]]; // Derives PDA using seed "mint" and bump seed
        let signer = [&seeds[..]]; // Wraps seeds in the required format for signers

        // Define the token metadata structure
        let token_data: DataV2 = DataV2 {
            name: metadata.name, // Name of the token (from params)
            symbol: metadata.symbol, // Symbol of the token (from params)
            uri: metadata.uri, // Metadata URI (from params)
            seller_fee_basis_points: 0, // No royalty fees specified
            creators: None, // No specific creators set
            collection: None, // No collection linked
            uses: None, // No usage constraints
        };

        // Create the CPI (Cross-Program Invocation) context for creating metadata
        let metadata_ctx = CpiContext::new_with_signer(
            _ctx.accounts.token_metadata_program.to_account_info(), // Metadata program account
            CreateMetadataAccountsV3 {
                payer: _ctx.accounts.payer.to_account_info(), // Payer funding the transaction
                update_authority: _ctx.accounts.mint.to_account_info(), // Update authority set to mint
                mint: _ctx.accounts.mint.to_account_info(), // Mint account
                metadata: _ctx.accounts.metadata.to_account_info(), // Metadata account
                mint_authority: _ctx.accounts.mint.to_account_info(), // Mint authority
                system_program: _ctx.accounts.system_program.to_account_info(), // System program account
                rent: _ctx.accounts.rent.to_account_info(), // Rent sysvar account
            },
            &signer // Signer for the mint PDA
        );

        // Create the metadata account using the Metaplex program
        create_metadata_accounts_v3(metadata_ctx, token_data, false, true, None)?;

        // Log success message
        msg!("Token mint created successfully!");

        Ok(()) // Return success
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