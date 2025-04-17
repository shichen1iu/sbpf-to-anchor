use anchor_lang::prelude::*;
use solana_program::program_error::ProgramError;

mod error;
mod instructions;
mod states;
mod utils;

use instructions::*;
use states::*;

declare_id!("D9MxitF878nXCgeTyiUYXGjsC8hh55HY2RzVnMRLwJSJ");

#[program]
pub mod sbpf_to_anchor {
    use super::*;
}
