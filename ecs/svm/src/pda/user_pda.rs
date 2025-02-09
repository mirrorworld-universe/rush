use solana_program::pubkey::Pubkey;

/// Finds the [`UserPDA`] PDA with canonical bump
///
/// - Used for validating User seeds
/// - Used for identifying specific User account onchain
pub struct UserPDA {}

impl UserPDA {
    /// Tag seed for differentiating state
    pub const TAG: &'static str = "User";

    /// Find PDA for User State
    ///
    /// Also searches for canonical Bump Seed,
    /// hence is very expensive.
    ///
    /// If Bump Seed is available,
    /// use [`Self::create_pda`] instead
    ///
    /// Returns (PDA, Bump Seed)
    pub fn find_pda(
        program_id: &Pubkey,
        user_authority: &Pubkey,
        world_pda: &Pubkey,
        user_agent_salt: String,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                Self::TAG.as_bytes(),
                world_pda.as_ref(),
                user_authority.as_ref(),
                user_agent_salt.as_bytes(),
            ],
            program_id,
        )
    }

    /// Create PDA for User State
    ///
    /// Doesn't search for canonical Bump Seed.
    ///
    /// Cheaper and encouraged to use if Bump
    /// Seed is available.
    ///
    /// Returns PDA
    pub fn create_pda(
        program_id: &Pubkey,
        user_authority: &Pubkey,
        world_pda: &Pubkey,
        user_agent_salt: String,
        bump_seed: u8,
    ) -> Pubkey {
        // expects a valid set of seeds
        Pubkey::create_program_address(
            &[
                Self::TAG.as_bytes(),
                world_pda.as_ref(),
                user_authority.as_ref(),
                user_agent_salt.as_bytes(),
                &[bump_seed],
            ],
            program_id,
        )
        .expect("Invalid seeds")
    }
}
