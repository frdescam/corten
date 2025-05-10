use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum TokenInstructions {
    AskMint { amount: u64 },
    Instruction2,
    Instruction3,
}
