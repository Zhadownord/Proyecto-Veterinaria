use anchor_lang::prelude::*;

declare_id!("HPYiDfKNivKdtQhZ525UiuHHAw4HYjCuKsuZjVEeAzrk");

#[program]
pub mod veterinaria_program {
    use super::*;

    // =========================
    // CREAR VETERINARIA
    // =========================
    pub fn crear_veterinaria(
        ctx: Context<CrearVeterinaria>,
        nombre: String,
    ) -> Result<()> {

        let veterinaria = &mut ctx.accounts.veterinaria;

        veterinaria.nombre = nombre;
        veterinaria.owner = ctx.accounts.owner.key();
        veterinaria.mascotas = Vec::new();

        Ok(())
    }

    // =========================
    // AGREGAR MASCOTA
    // =========================
    pub fn agregar_mascota(
        ctx: Context<ModificarVeterinaria>,
        nombre: String,
        especie: String,
        dueno: String,
        edad: u8,
        vivo: bool,
    ) -> Result<()> {

        let veterinaria = &mut ctx.accounts.veterinaria;

        require!(
            veterinaria.mascotas.len() < 20,
            Errores::LimiteMascotasAlcanzado
        );

        let nueva_mascota = Mascota {
            nombre,
            especie,
            owner: dueno,
            edad,
            vivo,
        };

        veterinaria.mascotas.push(nueva_mascota);

        Ok(())
    }

    // =========================
    // VER MASCOTAS
    // =========================
    pub fn ver_mascotas(ctx: Context<ModificarVeterinaria>) -> Result<()> {

        msg!("Lista de mascotas:");
        msg!("{:#?}", ctx.accounts.veterinaria.mascotas);

        Ok(())
    }

    // =========================
    // ACTUALIZAR MASCOTA
    // =========================
    pub fn actualizar_mascota(
        ctx: Context<ModificarVeterinaria>,
        nombre: String,
        nueva_edad: u8,
        nuevo_estado: bool,
    ) -> Result<()> {

        let veterinaria = &mut ctx.accounts.veterinaria;

        if let Some(mascota) = veterinaria
            .mascotas
            .iter_mut()
            .find(|m| m.nombre == nombre)
        {
            mascota.edad = nueva_edad;
            mascota.vivo = nuevo_estado;

            Ok(())
        } else {
            Err(Errores::MascotaNoExiste.into())
        }
    }

    // =========================
    // ELIMINAR MASCOTA
    // =========================
    pub fn eliminar_mascota(
        ctx: Context<ModificarVeterinaria>,
        nombre: String,
    ) -> Result<()> {

        let veterinaria = &mut ctx.accounts.veterinaria;

        if let Some(pos) = veterinaria
            .mascotas
            .iter()
            .position(|m| m.nombre == nombre)
        {
            veterinaria.mascotas.remove(pos);
            Ok(())
        } else {
            Err(Errores::MascotaNoExiste.into())
        }
    }
}

//
// =========================
// CUENTA PRINCIPAL
// =========================
//

#[account]
#[derive(InitSpace)]
pub struct Veterinaria {

    #[max_len(50)]
    pub nombre: String,

    pub owner: Pubkey,

    #[max_len(20)]
    pub mascotas: Vec<Mascota>,
}

//
// =========================
// STRUCT MASCOTA
// =========================
//

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, Debug)]
pub struct Mascota {

    #[max_len(60)]
    pub nombre: String,

    #[max_len(60)]
    pub especie: String,

    #[max_len(60)]
    pub owner: String,

    pub edad: u8,
    pub vivo: bool,
}

//
// =========================
// CONTEXTOS
// =========================
//

#[derive(Accounts)]
pub struct CrearVeterinaria<'info> {

    #[account(
        init,
        payer = owner,
        space = 8 + Veterinaria::INIT_SPACE,
        seeds = [b"veterinaria", owner.key().as_ref()],
        bump
    )]
    pub veterinaria: Account<'info, Veterinaria>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarVeterinaria<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        has_one = owner
    )]
    pub veterinaria: Account<'info, Veterinaria>,
}

//
// =========================
// ERRORES
// =========================
//

#[error_code]
pub enum Errores {

    #[msg("La mascota no existe")]
    MascotaNoExiste,

    #[msg("Se alcanzó el límite máximo de mascotas")]
    LimiteMascotasAlcanzado,
}
