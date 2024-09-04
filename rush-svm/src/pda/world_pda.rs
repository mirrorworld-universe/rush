use solana_program::pubkey::Pubkey;

/// Finds the [`WorldPDA`] PDA with canonical bump
///
/// - Used for validating World seeds
/// - Used for identifying specific World account onchain
pub struct WorldPDA {}

impl WorldPDA {
    /// Tag seed for differentiating state
    pub const TAG: &'static str = "World";

    /// Find PDA for World State
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
        name: &str,
        description: &str,
        world_authority: &Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                Self::TAG.as_bytes(),
                name.as_bytes(),
                description.as_bytes(),
                world_authority.as_ref(),
            ],
            program_id,
        )
    }

    /// Create PDA for World State
    ///
    /// Doesn't search for canonical Bump Seed.
    ///
    /// Cheaper and encouraged to use if Bump
    /// Seed is available.
    ///
    /// Returns PDA
    pub fn create_pda(
        program_id: &Pubkey,
        name: &str,
        description: &str,
        world_authority: &Pubkey,
        bump_seed: u8,
    ) -> Pubkey {
        // expects a valid set of seeds
        Pubkey::create_program_address(
            &[
                Self::TAG.as_bytes(),
                name.as_bytes(),
                description.as_bytes(),
                world_authority.as_ref(),
                &[bump_seed],
            ],
            program_id,
        )
        .expect("Invalid seeds")
    }
}
