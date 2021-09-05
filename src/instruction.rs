// inside instruction.rs
pub enum EscrowInstruction {

    /// Starts the trade by crating and populatiing an escrow account and trasferring ownership of the given temp token account to the PDA
    /// 
    /// 
    /// Accounts expected
    /// 
    /// 0. '[signer]' The account of the person initializing the escrow
    /// 1. '[writable]' Temporary toke account that should bem created prior to this instruction and owned by the initializer
    /// 2. '[]' The initializer's token account for the token they will receive should the trade go through
    /// 3. '[writable]' The escrow account, it will hold all necessary info about the trade.
    /// 4. '[]' The rent sysvar
    /// 5. '[]' The token program
    InitEscrow {
        /// The amount party A expects to receive of token Y
        amount: u64
    }
}