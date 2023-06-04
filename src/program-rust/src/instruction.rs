use solana_program::{program_error::ProgramError};

#[derive(Debug)]
pub enum HelloInstruction {
    Increment,
    Decrement,
    Set(u32),
}

impl HelloInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {

    }

}