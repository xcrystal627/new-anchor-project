use anchor_lang::prelude::*;

declare_id!("96NKE5891N6Bi8eCBtzZ2YYA63dRCFfutCKxwwXx4FBh");

#[program]
pub mod new_anchor_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
