use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;
use spl_discriminator::{ArrayDiscriminator, SplDiscriminate};

// OPT-OUT: didn't use #[seeds()] because ShankAccount seeds
// helper attribute is buggy. PDA is generated offchain
// instead and seeds are validated

/// User State
///
/// User's PDA is also used as a CPI signer
/// for Proxied Rush Store transactions
#[derive(
    Clone,
    BorshSerialize,
    BorshDeserialize,
    Debug,
    Default,
    Eq,
    PartialEq,
    ShankAccount,
    SplDiscriminate,
)]
#[discriminator_hash_input("rush_proxy::state::User")]
pub struct User {
    /// Identifier for this specific structure
    pub discriminator: [u8; 8],
    pub user_authority: Pubkey,
    pub bump: u8,
}

impl User {
    /// Is `true` if User is initialized
    pub fn is_initialized(&self) -> bool {
        self.discriminator.as_slice() == User::SPL_DISCRIMINATOR_SLICE
    }

    /// Is `true` if User is uninitialized
    pub fn is_uninitialized(&self) -> bool {
        self.discriminator.as_slice() == ArrayDiscriminator::UNINITIALIZED.as_slice()
    }

    /// Create new User state
    pub fn new(user_authority: Pubkey, bump: u8) -> Self {
        Self {
            user_authority,
            bump,
            discriminator: User::SPL_DISCRIMINATOR.into(),
        }
    }
}
