use anchor_lang::prelude::*;

declare_id!("35o3vydxxPhLpXvRctidue72oe6SfgsLXyk2SwmtyTCU"); // Playground lo actualizará al compilar/desplegar

#[program]
pub mod solana_todo_app {
    use super::*;

    // 1. Inicializa el perfil del usuario (PDA padre)
    pub fn initialize_profile(ctx: Context<InitializeProfile>) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;
        profile.authority = ctx.accounts.authority.key();
        profile.task_count = 0;
        profile.bump = ctx.bumps.user_profile;
        Ok(())
    }

    // 2. Crea una nueva tarea vinculada al perfil (PDA hija)
    pub fn create_task(ctx: Context<CreateTask>, title: String) -> Result<()> {
        require!(title.chars().count() <= 64, TodoError::TitleTooLong);
        require!(!title.trim().is_empty(), TodoError::EmptyTitle);

        let profile = &mut ctx.accounts.user_profile;
        let task = &mut ctx.accounts.task_account;

        task.authority = ctx.accounts.authority.key();
        task.task_id = profile.task_count;
        task.title = title;
        task.completed = false;
        task.bump = ctx.bumps.task_account;

        profile.task_count = profile.task_count.checked_add(1).unwrap();
        Ok(())
    }

    // 3. Alterna el estado de la tarea (completada / pendiente)
    pub fn toggle_task(ctx: Context<UpdateTask>) -> Result<()> {
        let task = &mut ctx.accounts.task_account;
        task.completed = !task.completed;
        Ok(())
    }

    // 4. Elimina la tarea y devuelve la renta (SOL) al usuario
    pub fn delete_task(_ctx: Context<DeleteTask>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + UserProfile::INIT_SPACE,
        seeds = [b"user-profile", authority.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTask<'info> {
    #[account(
        mut,
        seeds = [b"user-profile", authority.key().as_ref()],
        bump = user_profile.bump,
        has_one = authority
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(
        init,
        payer = authority,
        space = 8 + TaskAccount::INIT_SPACE,
        seeds = [b"task", authority.key().as_ref(), &user_profile.task_count.to_le_bytes()],
        bump
    )]
    pub task_account: Account<'info, TaskAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTask<'info> {
    #[account(
        mut,
        seeds = [b"task", authority.key().as_ref(), &task_account.task_id.to_le_bytes()],
        bump = task_account.bump,
        has_one = authority
    )]
    pub task_account: Account<'info, TaskAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteTask<'info> {
    #[account(
        mut,
        close = authority,
        seeds = [b"task", authority.key().as_ref(), &task_account.task_id.to_le_bytes()],
        bump = task_account.bump,
        has_one = authority
    )]
    pub task_account: Account<'info, TaskAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct UserProfile {
    pub authority: Pubkey,
    pub task_count: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct TaskAccount {
    pub authority: Pubkey,
    pub task_id: u64,
    #[max_len(64)]
    pub title: String,
    pub completed: bool,
    pub bump: u8,
}

#[error_code]
pub enum TodoError {
    #[msg("El titulo no puede superar los 64 caracteres.")]
    TitleTooLong,
    #[msg("El titulo no puede estar vacio.")]
    EmptyTitle,
}
