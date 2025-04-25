use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnT");

#[program]
pub mod execution {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let tracking = &mut ctx.accounts.tracking;
        tracking.authority = ctx.accounts.authority.key();
        tracking.total_executions = 0;
        tracking.active = true;
        Ok(())
    }

    pub fn start_execution(
        ctx: Context<StartExecution>,
        agent_id: Pubkey,
        context_id: Pubkey,
        execution_type: ExecutionType,
    ) -> Result<()> {
        let tracking = &mut ctx.accounts.tracking;
        let execution = &mut ctx.accounts.execution;
        
        execution.agent = agent_id;
        execution.context = context_id;
        execution.authority = ctx.accounts.authority.key();
        execution.start_time = Clock::get()?.unix_timestamp;
        execution.end_time = 0;
        execution.status = ExecutionStatus::InProgress;
        execution.execution_type = execution_type;
        execution.compute_units_consumed = 0;
        
        tracking.total_executions += 1;
        
        Ok(())
    }

    pub fn complete_execution(
        ctx: Context<CompleteExecution>,
        compute_units: u64,
    ) -> Result<()> {
        let execution = &mut ctx.accounts.execution;
        
        execution.end_time = Clock::get()?.unix_timestamp;
        execution.status = ExecutionStatus::Completed;
        execution.compute_units_consumed = compute_units;
        
        Ok(())
    }

    pub fn fail_execution(
        ctx: Context<CompleteExecution>,
        error_code: u16,
        error_message: String,
    ) -> Result<()> {
        let execution = &mut ctx.accounts.execution;
        
        execution.end_time = Clock::get()?.unix_timestamp;
        execution.status = ExecutionStatus::Failed;
        execution.error_code = Some(error_code);
        execution.error_message = Some(error_message);
        
        Ok(())
    }
}

#[account]
pub struct ExecutionTracking {
    pub authority: Pubkey,
    pub total_executions: u64,
    pub active: bool,
}

#[account]
pub struct Execution {
    pub agent: Pubkey,
    pub context: Pubkey,
    pub authority: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub status: ExecutionStatus,
    pub execution_type: ExecutionType,
    pub compute_units_consumed: u64,
    pub error_code: Option<u16>,
    pub error_message: Option<String>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ExecutionStatus {
    InProgress,
    Completed,
    Failed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ExecutionType {
    AgentInvocation,
    ContextOperation,
    SystemFunction,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 1)]
    pub tracking: Account<'info, ExecutionTracking>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StartExecution<'info> {
    #[account(mut)]
    pub tracking: Account<'info, ExecutionTracking>,
    #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 8 + 8 + 1 + 1 + 8 + 2 + 200)]
    pub execution: Account<'info, Execution>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteExecution<'info> {
    #[account(mut)]
    pub execution: Account<'info, Execution>,
    #[account(mut)]
    pub authority: Signer<'info>,
}
