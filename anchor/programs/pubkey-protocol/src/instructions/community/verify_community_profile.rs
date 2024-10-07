use anchor_lang::prelude::*;

use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct VerifyProfileForCommunity<'info> {
    #[account(mut)]
    pub community: Account<'info, Community>,
    #[account(mut)]
    pub profile: Account<'info, Profile>,
    #[account(constraint = authority.key() == community.authority @ PubkeyProfileError::UnauthorizedCommunityAction)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub fee_payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn verify_community_profile(ctx: Context<VerifyProfileForCommunity>) -> Result<()> {
    let community = &mut ctx.accounts.community;
    let profile = &mut ctx.accounts.profile;
    let authority = &ctx.accounts.authority;

    community.add_profile_verification(profile.key(), authority.key())?;

    profile.add_community_verification(community.key(), authority.key())?;

    Ok(())
}
