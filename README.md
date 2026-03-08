# HER DAO Member Registry 🌐

> On-chain member registry for HER DAO Venezuela — built on Solana using Rust and Anchor. A decentralized way to register, update and manage community members without intermediaries.

---

## 📌 Program ID (Devnet)
`DUoLxLrznZjvbTYcLFRVPC2VcRTBWjEmavASCxTD7jLb`

🔍 [Ver en Solana Explorer](https://explorer.solana.com/address/DUoLxLrznZjvbTYcLFRVPC2VcRTBWjEmavASCxTD7jLb?cluster=devnet)

---

## 🇺🇸 English

### What is this?

A decentralized member registry built on the Solana blockchain. Each member profile is stored in a unique on-chain account (PDA) linked to their wallet — meaning only the owner can create, update, or deactivate their own profile. No central database, no intermediaries.

### Features

- ✅ Register a member profile (name, role, bio)
- ✅ Update profile information
- ✅ Deactivate a member (soft delete)
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
| `create_member` | Creates a new on-chain member profile |
| `update_member` | Updates name, role, and bio |
| `delete_member` | Deactivates the member (sets active = false) |

### Account Structure

```rust
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
5. Use the **Test** panel to call `create_member` with your data

### Why this project?

HER DAO Venezuela is a community focused on women in tech and blockchain in Latin America. This registry is a proof of concept for managing community membership on-chain — transparent, permissionless, and owned by each member.

---

## 🇻🇪 Español

### ¿Qué es esto?

Un registro descentralizado de miembros construido en la blockchain de Solana. Cada perfil se guarda en una cuenta única on-chain (PDA) vinculada a la wallet del miembro — lo que significa que solo el dueño puede crear, actualizar o desactivar su propio perfil. Sin base de datos central, sin intermediarios.

### Funcionalidades

- ✅ Registrar un perfil de miembro (nombre, rol, bio)
- ✅ Actualizar la información del perfil
- ✅ Desactivar un miembro (soft delete)
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
| `create_member` | Crea un nuevo perfil de miembro on-chain |
| `update_member` | Actualiza nombre, rol y bio |
| `delete_member` | Desactiva al miembro (active = false) |

### Estructura de la cuenta

```rust
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
5. Usa el panel **Test** para llamar `create_member` con tus datos

### ¿Por qué este proyecto?

HER DAO Venezuela es una comunidad enfocada en mujeres en tecnología y blockchain en América Latina. Este registro es una prueba de concepto para gestionar la membresía de la comunidad on-chain — transparente, sin permisos centralizados y con propiedad real de cada miembro.

---

## Author

**Arlette Salas** — Founder, H.E.R. DAO Venezuela  
Solana Developer Certification — WayLearn Bootcamp 2026
