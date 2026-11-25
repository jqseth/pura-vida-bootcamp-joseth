use super::{EnvasesRetornables, EnvasesRetornablesClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn registra_y_devuelve_un_deposito() {
    let env = Env::default();
    env.mock_all_auths();
    let id_contrato = env.register(EnvasesRetornables, ());
    let contrato = EnvasesRetornablesClient::new(&env, &id_contrato);
    let encargado = Address::generate(&env);
    let cliente = Address::generate(&env);

    contrato.inicializar(&encargado);
    contrato.registrar(&1, &cliente, &500);
    let antes = contrato.consultar(&1).unwrap();
    assert_eq!(antes.cliente, cliente);
    assert_eq!(antes.monto_colones, 500);
    assert!(!antes.devuelto);

    contrato.devolver(&1);
    assert!(contrato.consultar(&1).unwrap().devuelto);
}

#[test]
fn devuelve_none_si_id_no_existe() {
    let env = Env::default();
    let id_contrato = env.register(EnvasesRetornables, ());
    let contrato = EnvasesRetornablesClient::new(&env, &id_contrato);
    assert_eq!(contrato.consultar(&99), None);
}

#[test]
#[should_panic]
fn no_permite_id_repetido() {
    let env = Env::default();
    env.mock_all_auths();
    let id_contrato = env.register(EnvasesRetornables, ());
    let contrato = EnvasesRetornablesClient::new(&env, &id_contrato);
    let encargado = Address::generate(&env);
    let cliente = Address::generate(&env);
    contrato.inicializar(&encargado);
    contrato.registrar(&1, &cliente, &500);
    contrato.registrar(&1, &cliente, &500);
}

#[test]
#[should_panic]
fn no_permite_devolucion_doble() {
    let env = Env::default();
    env.mock_all_auths();
    let id_contrato = env.register(EnvasesRetornables, ());
    let contrato = EnvasesRetornablesClient::new(&env, &id_contrato);
    let encargado = Address::generate(&env);
    let cliente = Address::generate(&env);
    contrato.inicializar(&encargado);
    contrato.registrar(&1, &cliente, &500);
    contrato.devolver(&1);
    contrato.devolver(&1);
}

#[test]
#[should_panic]
fn rechaza_montos_no_positivos() {
    let env = Env::default();
    env.mock_all_auths();
    let id_contrato = env.register(EnvasesRetornables, ());
    let contrato = EnvasesRetornablesClient::new(&env, &id_contrato);
    let encargado = Address::generate(&env);
    let cliente = Address::generate(&env);
    contrato.inicializar(&encargado);
    contrato.registrar(&2, &cliente, &0);
}
