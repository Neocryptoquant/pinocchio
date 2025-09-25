use solana_account_view::AccountView;
use solana_address::Address;
use solana_instruction_view::{
    cpi::{invoke_signed, Signer},
    AccountPrivilege, InstructionView,
};
use solana_program_error::ProgramResult;

/// Thaw a Frozen account using the Mint's freeze authority
///
/// ### Accounts:
///   0. `[WRITE]` The account to thaw.
///   1. `[]` The token mint.
///   2. `[SIGNER]` The mint freeze authority.
pub struct ThawAccount<'a, 'b> {
    /// Token Account to thaw.
    pub account: &'a AccountView,
    /// Mint Account.
    pub mint: &'a AccountView,
    /// Mint Freeze Authority Account
    pub freeze_authority: &'a AccountView,
    /// Token Program
    pub token_program: &'b Address,
}

impl ThawAccount<'_, '_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    #[inline(always)]
    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountPrivilege; 3] = [
            AccountPrivilege::writable(self.account.key()),
            AccountPrivilege::readonly(self.mint.key()),
            AccountPrivilege::readonly_signer(self.freeze_authority.key()),
        ];

        let instruction = InstructionView {
            program_id: self.token_program,
            accounts: &account_metas,
            data: &[11],
        };

        invoke_signed(
            &instruction,
            &[self.account, self.mint, self.freeze_authority],
            signers,
        )
    }
}
