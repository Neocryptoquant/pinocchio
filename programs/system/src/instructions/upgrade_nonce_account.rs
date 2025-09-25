use solana_account_view::AccountView;
use solana_instruction_view::{cpi::invoke, AccountPrivilege, InstructionView};
use solana_program_error::ProgramResult;

/// One-time idempotent upgrade of legacy nonce versions in order to bump
/// them out of chain blockhash domain.
///
/// ### Accounts:
///   0. `[WRITE]` Nonce account
pub struct UpgradeNonceAccount<'a> {
    /// Nonce account.
    pub account: &'a AccountView,
}

impl UpgradeNonceAccount<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        // account metadata
        let account_metas: [AccountPrivilege; 1] = [AccountPrivilege::writable(self.account.key())];

        // instruction
        let instruction = InstructionView {
            program_id: &crate::ID,
            accounts: &account_metas,
            data: &[12],
        };

        invoke(&instruction, &[self.account])
    }
}
