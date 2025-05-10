use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum TokenInstructions {
    AskMint,
    Instruction2,
    Instruction3,
}
