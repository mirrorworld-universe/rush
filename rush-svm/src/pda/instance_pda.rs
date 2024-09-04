use solana_program::pubkey::Pubkey;

/// Finds the [`InstancePDA`] PDA with canonical bump
///
/// - Used for validating Instance seeds
/// - Used for identifying specific Instance
pub struct InstancePDA {}

impl InstancePDA {
    /// Tag seed for differentiating state
    pub const TAG: &'static str = "Instance";

    /// Find PDA for Instance State
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
        world_pda: &Pubkey,
        region: &str,
        entity: &str,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                Self::TAG.as_bytes(),
                world_pda.as_ref(),
                region.as_bytes(),
                entity.as_bytes(),
            ],
            program_id,
        )
    }

    /// Create PDA for Instance State
    ///
    /// Doesn't search for canonical Bump Seed.
    ///
    /// Cheaper and encouraged to use if Bump
    /// Seed is available.
    ///
    /// Returns PDA
    pub fn create_pda(
        program_id: &Pubkey,
        world_pda: &Pubkey,
        region: &str,
        entity: &str,
        bump_seed: u8,
    ) -> Pubkey {
        // expects a valid set of seeds
        Pubkey::create_program_address(
            &[
                Self::TAG.as_bytes(),
                world_pda.as_ref(),
                region.as_bytes(),
                entity.as_bytes(),
                &[bump_seed],
            ],
            program_id,
        )
        .expect("Invalid seeds")
    }
}
