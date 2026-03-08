# HER DAO Member Registry 🌐

> On-chain member registry for H.E.R. DAO Venezuela 🇻🇪 — built on Solana using Rust and Anchor. A decentralized way to register, update and manage community members without intermediaries.

---

## 📌 Program ID (Devnet)
`DUoLxLrznZjvbTYcLFRVPC2VcRTBWjEmavASCxTD7jLb`

🔍 [View in Solana Explorer](https://explorer.solana.com/address/DUoLxLrznZjvbTYcLFRVPC2VcRTBWjEmavASCxTD7jLb?cluster=devnet)

---

## 🇺🇸 English

### What is this?

A decentralized member registry built on the Solana blockchain. Each member profile is stored in a unique on-chain account (PDA) linked to their wallet — meaning only the owner can create, update, or deactivate their own profile. No central database, no intermediaries.

### Features

- ✅ Initialize a global registry with a member counter (called once)
- ✅ Register a member profile (name, role, bio)
- ✅ Update profile information
- ✅ Deactivate a member (soft delete)
- ✅ Global counter to track total active members
- ✅ Each profile is a PDA derived from the member's wallet
- ✅ Built with Rust + Anchor on Solana Devnet

### Tech Stack

| Tool | Purpose |
|------|---------|
| Rust | Smart contract language |
| Anchor | Solana development framework |
| Solana Devnet | Test blockchain network |
| Solana Playground | Browser-based IDE |

### Program Instructions

| Instruction | Description |
|-------------|-------------|
| `initialize_registry` | Initializes the global member counter (called once) |
| `create_member` | Creates a new on-chain member profile |
| `update_member` | Updates name, role, and bio |
| `delete_member` | Deactivates the member (sets active = false) |

### Account Structure

```rust
// Global account — one per program
pub struct RegistryAccount {
    pub authority: Pubkey,   // Registry initializer
    pub total_members: u64,  // Active member count
}

// Individual account — one per member
pub struct MemberAccount {
    pub authority: Pubkey,   // Wallet owner
    pub name: String,        // Max 50 chars
    pub role: String,        // Max 50 chars
    pub bio: String,         // Max 200 chars
    pub active: bool,        // Active status
    pub created_at: i64,     // Unix timestamp
}
```

### How to use

1. Open [Solana Playground](https://beta.solpg.io)
2. Create a new Anchor project and paste the code from `src/lib.rs`
3. Connect your wallet (Phantom recommended) on Devnet
4. Click **Build** → **Deploy**
5. Call `initialize_registry` first (only once)
6. Use the **Test** panel to call `create_member` with your data

### Why this project?

H.E.R. DAO Venezuela 🇻🇪 is a community focused on women in tech and blockchain in Latin America. This registry is a proof of concept for managing community membership on-chain — transparent, permissionless, and owned by each member.

### 🗺️ Roadmap

**Phase 1 — Registry improvements**
- Add social media fields (X, GitHub,LinkedIn)
- Add location field (city)

**Phase 2 — Basic governance**
- Admin-based member verification system
- Role-based permissions (admin, member, collaborator)
- On-chain change history

**Phase 3 — Ecosystem integration**
- Membership NFT linked to member profile
- On-chain voting for DAO decisions
- Next.js frontend to interact with the program

---

## 🇻🇪 Español

### ¿Qué es esto?

Un registro descentralizado de miembros construido en la blockchain de Solana. Cada perfil se guarda en una cuenta única on-chain (PDA) vinculada a la wallet del miembro — lo que significa que solo el dueño puede crear, actualizar o desactivar su propio perfil. Sin base de datos central, sin intermediarios.

### Funcionalidades

- ✅ Inicializar un registro global con contador de miembros (se llama una vez)
- ✅ Registrar un perfil de miembro (nombre, rol, bio)
- ✅ Actualizar la información del perfil
- ✅ Desactivar un miembro (soft delete)
- ✅ Contador global para llevar el total de miembros activos
- ✅ Cada perfil es una PDA derivada de la wallet del miembro
- ✅ Construido con Rust + Anchor en Solana Devnet

### Stack tecnológico

| Herramienta | Uso |
|-------------|-----|
| Rust | Lenguaje para el contrato inteligente |
| Anchor | Framework de desarrollo en Solana |
| Solana Devnet | Red de pruebas de Solana |
| Solana Playground | IDE en el navegador |

### Instrucciones del programa

| Instrucción | Descripción |
|-------------|-------------|
| `initialize_registry` | Inicializa el contador global de miembros (se llama una vez) |
| `create_member` | Crea un nuevo perfil de miembro on-chain |
| `update_member` | Actualiza nombre, rol y bio |
| `delete_member` | Desactiva al miembro (active = false) |

### Estructura de las cuentas

```rust
// Cuenta global — existe una sola para todo el programa
pub struct RegistryAccount {
    pub authority: Pubkey,   // Quien inicializó el registro
    pub total_members: u64,  // Contador de miembros activos
}

// Cuenta individual — existe una por cada miembro
pub struct MemberAccount {
    pub authority: Pubkey,   // Dueño de la wallet
    pub name: String,        // Máx 50 caracteres
    pub role: String,        // Máx 50 caracteres
    pub bio: String,         // Máx 200 caracteres
    pub active: bool,        // Estado activo
    pub created_at: i64,     // Timestamp Unix
}
```

### Cómo usarlo

1. Abre [Solana Playground](https://beta.solpg.io)
2. Crea un nuevo proyecto Anchor y pega el código de `src/lib.rs`
3. Conecta tu wallet (se recomienda Phantom) en Devnet
4. Click en **Build** → **Deploy**
5. Llama primero a `initialize_registry` (solo una vez)
6. Usa el panel **Test** para llamar `create_member` con tus datos

### ¿Por qué este proyecto?

H.E.R. DAO Venezuela 🇻🇪 es una comunidad enfocada en mujeres en tecnología y blockchain en América Latina. Este registro es una prueba de concepto para gestionar la membresía de la comunidad on-chain — transparente, sin permisos centralizados y con propiedad real de cada miembro.

### 🗺️ Roadmap

**Fase 1 — Mejoras al registro**
- Agregar campo de redes sociales (X, GitHub,LinkedIn)
- Agregar campo de ciudad

**Fase 2 — Gobernanza básica**
- Sistema de verificación de miembros por un admin
- Roles con permisos distintos (admin, miembro, colaborador)
- Historial de cambios on-chain

**Fase 3 — Integración con el ecosistema**
- NFT de membresía vinculado al perfil
- Votación on-chain para decisiones de la DAO
- Frontend con Next.js para interactuar con el programa

---

## Author

**Arlette Salas** — Founder, H.E.R. DAO Venezuela 🇻🇪
Solana Developer Certification — WayLearn Bootcamp 2026
