use solana_account_view::AccountView;
use solana_address::Address;
use solana_instruction_view::{cpi::invoke, AccountPrivilege, InstructionView};
use solana_program_error::ProgramResult;

/// Given a native token account updates its amount field based
/// on the account's underlying `lamports`.
///
/// ### Accounts:
///   0. `[WRITE]`  The native token account to sync with its underlying
///      lamports.
pub struct SyncNative<'a, 'b> {
    /// Native Token Account
    pub native_token: &'a AccountView,
    /// Token Program
    pub token_program: &'b Address,
}

impl SyncNative<'_, '_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        // account metadata
        let account_metas: [AccountPrivilege; 1] =
            [AccountPrivilege::writable(self.native_token.key())];

        let instruction = InstructionView {
            program_id: self.token_program,
            accounts: &account_metas,
            data: &[17],
        };

        invoke(&instruction, &[self.native_token])
    }
}
