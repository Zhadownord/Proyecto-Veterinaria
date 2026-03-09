use anchor_lang::prelude::*;

// Identificador único del programa en Solana
declare_id!("9j7ppCah2Ss5CbbjqVbXeGc2SqGBr5tfgJpavUWDFgu1");

#[program]
pub mod cuenta_gastos {
    use super::*;

    // ------------------- CREAR CUENTA -------------------
    /// Inicializa una nueva cuenta de gastos asociada a un owner.
    /// Esta cuenta contendrá un vector vacío de gastos.
    pub fn crear_cuenta(ctx: Context<NuevaCuentaGastos>) -> Result<()> {
        let cuenta = &mut ctx.accounts.cuenta_gastos;
        cuenta.owner = ctx.accounts.owner.key();
        cuenta.gasto = Vec::new();
        Ok(())
    }

    // ------------------- CREAR GASTO -------------------
    /// Añade un nuevo gasto dentro de la cuenta ya creada.
    /// Recibe descripción, monto y fecha como parámetros.
    pub fn add_gasto(ctx: Context<NuevoGasto>,descripcion: String,monto: u64,fecha: i64,) -> Result<()> {
        let cuenta = &mut ctx.accounts.cuenta_gastos;
        // Validación: solo el owner puede añadir gastos
        require_keys_eq!(cuenta.owner, ctx.accounts.owner.key(), ErrorCode::Unauthorized);

        let nuevo_gasto = Gasto {
            descripcion,
            monto,
            fecha,
            owner: ctx.accounts.owner.key(),
        };

        cuenta.gasto.push(nuevo_gasto);
        msg!("Gasto añadido!");
        Ok(())
    }

    // ------------------- CONSULTAR GASTOS -------------------
    /// Muestra en el log todos los gastos almacenados en la cuenta.
    /// Nota: en Solana no se devuelven datos directamente, se consultan desde el cliente.
    pub fn consultar_gasto(ctx: Context<NuevoGasto>) -> Result<()> {
        require!(
            ctx.accounts.cuenta_gastos.owner == ctx.accounts.owner.key(),
            ErrorCode::Unauthorized
        );
        msg!("La lista de gastos es: {:#?}", ctx.accounts.cuenta_gastos.gasto);
        Ok(())
    }

    // ------------------- ACTUALIZAR GASTO -------------------
    /// Modifica un gasto existente identificado por su índice en el vector.
    pub fn update_gasto(ctx: Context<NuevoGasto>,index: u32,descripcion: String,monto: u64, fecha: i64, ) -> Result<()> {
        let cuenta = &mut ctx.accounts.cuenta_gastos;
        require_keys_eq!(cuenta.owner, ctx.accounts.owner.key(), ErrorCode::Unauthorized);

        let idx = index as usize;
        require!(idx < cuenta.gasto.len(), ErrorCode::IndexOutOfBounds);

        cuenta.gasto[idx].descripcion = descripcion;
        cuenta.gasto[idx].monto = monto;
        cuenta.gasto[idx].fecha = fecha;
        Ok(())
    }

    // ------------------- ELIMINAR GASTO -------------------
    /// Elimina un gasto del vector por índice.
    pub fn delete_gasto(ctx: Context<NuevoGasto>, index: u32) -> Result<()> {
        let cuenta = &mut ctx.accounts.cuenta_gastos;
        require_keys_eq!(cuenta.owner, ctx.accounts.owner.key(), ErrorCode::Unauthorized);

        let idx = index as usize;
        require!(idx < cuenta.gasto.len(), ErrorCode::IndexOutOfBounds);

        cuenta.gasto.remove(idx);
        Ok(())
    }
}

// ------------------- DEFINICIÓN DE CUENTAS -------------------
#[account]
#[derive(InitSpace)]
pub struct CuentaGastos {
    owner: Pubkey,              // Dueño de la cuenta
    #[max_len(50)]
    gasto: Vec<Gasto>,          // Lista de gastos (máx. 50)
}

// ------------------- STRUCT DE GASTO -------------------
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Gasto {
    #[max_len(60)]
    descripcion: String,        // Texto descriptivo del gasto
    monto: u64,                 // Cantidad gastada
    fecha: i64,                 // Timestamp del gasto
    owner: Pubkey,              // Dueño que creó el gasto
}

// ------------------- CONTEXTOS -------------------
#[derive(Accounts)]
pub struct NuevaCuentaGastos<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,   // Usuario que paga la creación
    #[account(
        init,
        payer = owner,
        space = 8 + CuentaGastos::INIT_SPACE, // 8 bytes extra para el discriminator
        seeds = [b"cuentaGastos", owner.key().as_ref()],
        bump
    )]
    pub cuenta_gastos: Account<'info, CuentaGastos>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoGasto<'info> {
    pub owner: Signer<'info>,   // Usuario que modifica la cuenta
    #[account(mut)]
    pub cuenta_gastos: Account<'info, CuentaGastos>,
}

// ------------------- ERRORES PERSONALIZADOS -------------------
#[error_code]
pub enum ErrorCode {
    #[msg("No autorizado")]
    Unauthorized,
    #[msg("Índice fuera de rango")]
    IndexOutOfBounds,
}
