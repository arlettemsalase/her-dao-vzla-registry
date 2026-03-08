// Importamos las herramientas de Anchor que necesitamos para construir el programa
use anchor_lang::prelude::*;

// Este es el ID único de tu programa en Solana.
// Solana Playground lo reemplaza automáticamente cuando haces Build+Deploy.
declare_id!("DUoLxLrznZjvbTYcLFRVPC2VcRTBWjEmavASCxTD7jLb");

// #[program] le dice a Anchor: "aquí adentro van todas las acciones que puede hacer el programa"
#[program]
pub mod her_dao_registry {
    use super::*;

    // -------------------------------------------------------
    // INITIALIZE REGISTRY — Configuración inicial del programa
    // Esta función se llama UNA SOLA VEZ cuando se despliega el programa.
    // Crea la cuenta global que llevará el conteo de todos los miembros.
    // Sin este paso, create_member fallará porque busca esta cuenta y no existe.
    // -------------------------------------------------------
    pub fn initialize_registry(ctx: Context<InitializeRegistry>) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.total_members = 0;         // El contador empieza en cero
        registry.authority = ctx.accounts.authority.key(); // Guardamos quién inicializó el registro
        msg!("Registro inicializado!");
        Ok(())
    }

    // -------------------------------------------------------
    // CREATE — Registrar un nuevo miembro
    // "ctx" = contexto (contiene las cuentas involucradas)
    // "name", "role", "bio" = los datos que el usuario envía
    // -------------------------------------------------------
    pub fn create_member(
        ctx: Context<CreateMember>,
        name: String,
        role: String,
        bio: String,
    ) -> Result<()> {
        // Validaciones: si el texto es muy largo, el programa lanza un error
        // y no guarda nada. Esto protege el espacio en blockchain.
        require!(name.len() <= 50, RegistryError::NameTooLong);
        require!(role.len() <= 50, RegistryError::RoleTooLong);
        require!(bio.len() <= 200, RegistryError::BioTooLong);

        // Accedemos a la cuenta del miembro para escribir en ella
        let member = &mut ctx.accounts.member;

        // Guardamos quién es el dueño de este perfil (su dirección de wallet)
        member.authority = ctx.accounts.authority.key();

        // Guardamos los datos enviados
        member.name = name;
        member.role = role;
        member.bio = bio;

        // El miembro empieza activo
        member.active = true;

        // Guardamos la fecha/hora exacta de creación (en formato Unix timestamp)
        member.created_at = Clock::get()?.unix_timestamp;

        // Incrementamos el contador global de miembros
        let registry = &mut ctx.accounts.registry;
        registry.total_members += 1;

        // msg! es como un console.log — aparece en los logs de la transacción
        msg!("Miembro registrado: {}. Total: {}", member.name, registry.total_members);
        Ok(()) // Todo salió bien
    }

    // -------------------------------------------------------
    // READ — No necesita función propia.
    // En Solana, leer una cuenta se hace directamente desde el cliente.
    // El dato ya está guardado en la blockchain, solo hay que consultarlo.
    // -------------------------------------------------------

    // -------------------------------------------------------
    // UPDATE — Actualizar el perfil de un miembro existente
    // Solo el dueño original (has_one = authority) puede modificarlo.
    // -------------------------------------------------------
    pub fn update_member(
        ctx: Context<UpdateMember>,
        name: String,
        role: String,
        bio: String,
    ) -> Result<()> {
        require!(name.len() <= 50, RegistryError::NameTooLong);
        require!(role.len() <= 50, RegistryError::RoleTooLong);
        require!(bio.len() <= 200, RegistryError::BioTooLong);

        let member = &mut ctx.accounts.member;
        member.name = name;
        member.role = role;
        member.bio = bio;

        msg!("Perfil actualizado: {}", member.name);
        Ok(())
    }

    // -------------------------------------------------------
    // DELETE — Desactivar un miembro (soft delete)
    // Se archivan en vez de eliminar.
    // También resta 1 miembro del contador global.
    // -------------------------------------------------------
    pub fn delete_member(ctx: Context<DeleteMember>) -> Result<()> {
        let member = &mut ctx.accounts.member;
        member.active = false;

        // Decrementamos el contador, con protección para no llegar a números negativos
        let registry = &mut ctx.accounts.registry;
        if registry.total_members > 0 {
            registry.total_members -= 1;
        }

        msg!("Miembro desactivado: {}. Total: {}", member.name, registry.total_members);
        Ok(())
    }
}

// -------------------------------------------------------
// ACCOUNTS — Aquí definimos qué cuentas necesita cada función.
// -------------------------------------------------------

// Cuentas para initialize_registry
#[derive(Accounts)]
pub struct InitializeRegistry<'info> {
    #[account(
        init,               // Crear esta cuenta nueva
        payer = authority,  // Quien paga el "rent"
        space = RegistryAccount::SIZE,
        seeds = [b"registry"], // PDA derivada solo de "registry" — es única para todo el programa
        bump
    )]
    pub registry: Account<'info, RegistryAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Cuentas para create_member
#[derive(Accounts)]
pub struct CreateMember<'info> {
    #[account(
        init,               // Crear esta cuenta nueva
        payer = authority,  // Quien paga el "rent" (costo de guardar datos en Solana)
        space = MemberAccount::SIZE,
        seeds = [b"member", authority.key().as_ref()], // PDA: dirección derivada de "member" + tu wallet
        bump
    )]
    pub member: Account<'info, MemberAccount>,

    // La cuenta global del registro — la actualizamos para sumar +1 al contador
    #[account(
        mut,
        seeds = [b"registry"],
        bump
    )]
    pub registry: Account<'info, RegistryAccount>,

    #[account(mut)] // mut = puede cambiar (porque va a pagar y firmar)
    pub authority: Signer<'info>, // La persona que firma la transacción (tú)

    pub system_program: Program<'info, System>, 
}

// Cuentas para update_member
#[derive(Accounts)]
pub struct UpdateMember<'info> {
    #[account(
        mut,
        seeds = [b"member", authority.key().as_ref()], // Misma PDA que al crear
        bump,
        has_one = authority // Solo el dueño original puede modificar
    )]
    pub member: Account<'info, MemberAccount>,

    pub authority: Signer<'info>,
}

// Cuentas para delete_member
#[derive(Accounts)]
pub struct DeleteMember<'info> {
    #[account(
        mut,
        seeds = [b"member", authority.key().as_ref()],
        bump,
        has_one = authority // Solo el dueño puede desactivar
    )]
    pub member: Account<'info, MemberAccount>,

    // La cuenta global del registro — la actualizamos para restar -1 al contador
    #[account(
        mut,
        seeds = [b"registry"],
        bump
    )]
    pub registry: Account<'info, RegistryAccount>,

    pub authority: Signer<'info>,
}

// -------------------------------------------------------
// DATA STRUCTURES — Las estructuras de datos
// -------------------------------------------------------

// Cuenta global — existe una sola para todo el programa
// Guarda el estado general del registro
#[account]
pub struct RegistryAccount {
    pub authority: Pubkey,  // Quien inicializó el registro (32 bytes)
    pub total_members: u64, // Contador de miembros activos (8 bytes)
}

impl RegistryAccount {
    pub const SIZE: usize = 8  // discriminator (identificador interno de Anchor, siempre 8)
        + 32                    // authority
        + 8;                    // total_members (u64 = 8 bytes)
}

// Cuenta individual — existe una por cada miembro registrado
#[account]
pub struct MemberAccount {
    pub authority: Pubkey, // Dirección de wallet del dueño (32 bytes)
    pub name: String,      // Nombre del miembro (máx 50 caracteres)
    pub role: String,      // Rol en la comunidad (máx 50 caracteres)
    pub bio: String,       // Descripción corta (máx 200 caracteres)
    pub active: bool,      // ¿Está activo? true/false
    pub created_at: i64,   // Fecha de creación en Unix timestamp
}

impl MemberAccount {
    // Calculamos el espacio total que ocupará esta cuenta en Solana.
    // Solana cobra "rent" según el espacio, así que hay que ser exactos.
    pub const SIZE: usize = 8    // discriminator (identificador interno de Anchor, siempre 8)
        + 32                      // authority (Pubkey siempre es 32 bytes)
        + 4 + 50                  // name: 4 bytes para el largo del String + 50 del contenido
        + 4 + 50                  // role: igual
        + 4 + 200                 // bio: igual pero 200 caracteres
        + 1                       // active: bool = 1 byte
        + 8;                      // created_at: i64 = 8 bytes
}

// -------------------------------------------------------
// ERRORS — Mensajes de error personalizados
// -------------------------------------------------------
#[error_code]
pub enum RegistryError {
    #[msg("El nombre no puede superar 50 caracteres")]
    NameTooLong,
    #[msg("El rol no puede superar 50 caracteres")]
    RoleTooLong,
    #[msg("La bio no puede superar 200 caracteres")]
    BioTooLong,
}
