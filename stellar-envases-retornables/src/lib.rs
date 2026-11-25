#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

/// Contrato educativo: registra depósitos, no custodia ni transfiere fondos.
#[contract]
pub struct EnvasesRetornables;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deposito {
    pub cliente: Address,
    pub monto_colones: i128,
    pub devuelto: bool,
}

#[contracttype]
pub enum Clave {
    Encargado,
    Deposito(u32),
}

#[contractimpl]
impl EnvasesRetornables {
    /// Se configura una sola vez. El encargado autoriza los movimientos.
    pub fn inicializar(env: Env, encargado: Address) {
        assert!(!env.storage().instance().has(&Clave::Encargado), "ya inicializado");
        encargado.require_auth();
        env.storage().instance().set(&Clave::Encargado, &encargado);
    }

    /// Registra un depósito en colones (sin transferir dinero).
    pub fn registrar(env: Env, id: u32, cliente: Address, monto_colones: i128) {
        Self::autorizar_encargado(&env);
        assert!(monto_colones > 0, "el monto debe ser positivo");
        let clave = Clave::Deposito(id);
        assert!(!env.storage().persistent().has(&clave), "id ya registrado");
        let deposito = Deposito { cliente, monto_colones, devuelto: false };
        env.storage().persistent().set(&clave, &deposito);
    }

    /// Marca que el negocio devolvió el depósito al recibir el envase.
    pub fn devolver(env: Env, id: u32) {
        Self::autorizar_encargado(&env);
        let clave = Clave::Deposito(id);
        let mut deposito: Deposito = env.storage().persistent().get(&clave).expect("deposito inexistente");
        assert!(!deposito.devuelto, "deposito ya devuelto");
        deposito.devuelto = true;
        env.storage().persistent().set(&clave, &deposito);
    }

    /// Consulta pública del estado del depósito.
    pub fn consultar(env: Env, id: u32) -> Option<Deposito> {
        env.storage().persistent().get(&Clave::Deposito(id))
    }

    fn autorizar_encargado(env: &Env) {
        let encargado: Address = env.storage().instance().get(&Clave::Encargado).expect("falta inicializar");
        encargado.require_auth();
    }
}

#[cfg(test)]
mod test;
