use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod registry {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.authority = ctx.accounts.authority.key();
        registry.agent_count = 0;
        registry.context_count = 0;
        Ok(())
    }

    pub fn register_agent(ctx: Context<RegisterAgent>, name: String) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let agent = &mut ctx.accounts.agent;
        
        agent.authority = ctx.accounts.authority.key();
        agent.name = name;
        agent.active = true;
        agent.created_at = Clock::get()?.unix_timestamp;
        
        registry.agent_count += 1;
        
        Ok(())
    }

    pub fn register_context(ctx: Context<RegisterContext>, name: String) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let context = &mut ctx.accounts.context;
        
        context.authority = ctx.accounts.authority.key();
        context.name = name;
        context.active = true;
        context.created_at = Clock::get()?.unix_timestamp;
        
        registry.context_count += 1;
        
        Ok(())
    }

    pub fn grant_permission(ctx: Context<GrantPermission>) -> Result<()> {
        let permission = &mut ctx.accounts.permission;
        
        permission.agent = ctx.accounts.agent.key();
        permission.context = ctx.accounts.context.key();
        permission.granted_by = ctx.accounts.authority.key();
        permission.granted_at = Clock::get()?.unix_timestamp;
        permission.active = true;
        
        Ok(())
    }

    pub fn revoke_permission(ctx: Context<RevokePermission>) -> Result<()> {
        let permission = &mut ctx.accounts.permission;
        permission.active = false;
        Ok(())
    }
}

#[account]
pub struct Registry {
    pub authority: Pubkey,
    pub agent_count: u64,
    pub context_count: u64,
}

#[account]
pub struct Agent {
    pub authority: Pubkey,
    pub name: String,
    pub active: bool,
    pub created_at: i64,
}

#[account]
pub struct Context {
    pub authority: Pubkey,
    pub name: String,
    pub active: bool,
    pub created_at: i64,
}

#[account]
pub struct Permission {
    pub agent: Pubkey,
    pub context: Pubkey,
    pub granted_by: Pubkey,
    pub granted_at: i64,
    pub active: bool,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 8)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterAgent<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(init, payer = authority, space = 8 + 32 + 64 + 1 + 8)]
    pub agent: Account<'info, Agent>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterContext<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(init, payer = authority, space = 8 + 32 + 64 + 1 + 8)]
    pub context: Account<'info, Context>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GrantPermission<'info> {
    #[account(mut)]
    pub agent: Account<'info, Agent>,
    #[account(mut)]
    pub context: Account<'info, Context>,
    #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 8 + 1)]
    pub permission: Account<'info, Permission>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevokePermission<'info> {
    #[account(mut)]
    pub permission: Account<'info, Permission>,
    #[account(mut)]
    pub authority: Signer<'info>,
}
