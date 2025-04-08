fn main() {
    println!("Hello, world!");
}
use anchor_lang::prelude::*;

declare_id!("6rJJaiQXA8Urtfc8yNtdH7bUEBZMv4UyYns7DBHuTewm");


pub const ANCHOR_DISCRIMINATOR_SIZE : usize = 8;

#[program]


pub mod favorites{
    use super::*;

    pub fn set_favorites() -> Result<()> {
        msg!("Greetings from {}", Context.program_id);
        let user_public_key = context.user.account.key();
        msg!("User {user_public_key}'s favorite number is {number} thcontcteir favorite color is {color} and their hobbies are {hobbies:?}");

        context.accounts.favorites.set_inner(Favorites{
            number,
            color,
            hobbies,

        });
        Ok(())


    }
}
#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,
    #[max_len(50)]
    pub color: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}
#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + // discriminator
            8 + // u64 for number
            4 + 50 + // String length prefix (4 bytes) + max color string length
            4 + (5 * (4 + 50)) // Vec length prefix (4 bytes) + (max items * (string length prefix + max string length))
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

