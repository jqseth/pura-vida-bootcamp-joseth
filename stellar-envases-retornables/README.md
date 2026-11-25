# Envases Retornables CR ♻️ — Primer contrato inteligente en Stellar

**Autor:** Joseth Gamboa Guzmán  
**Bootcamp:** Pura Vida Bootcamp · Stellar × BAF × Oppia  
**Red prevista:** Stellar Testnet  
**Tecnologías:** Rust, Soroban SDK y Stellar CLI

## 1. Problema real y concreto

En pulperías, sodas y ferias del agricultor de Costa Rica se utilizan envases retornables. Un negocio puede cobrar, por ejemplo, **₡500 de depósito** al prestar un envase, y devolver ese monto cuando recibe el envase de vuelta. Si el control se lleva en papel o de memoria, es fácil confundir qué depósitos siguen pendientes o registrar dos veces la devolución.

**Pregunta a resolver:** ¿cómo llevar un registro sencillo y consultable de cuáles depósitos ya se devolvieron?

## 2. Solución

Este contrato inteligente funciona como **libro de registro** para un comercio pequeño. La persona encargada puede registrar un depósito indicando un identificador, la dirección pública del cliente y un monto en colones; luego marca el depósito como devuelto. Cualquiera puede consultar el estado mediante su identificador.

> **Alcance importante:** este contrato **NO recibe, custodia ni devuelve dinero o tokens**. El monto en colones es únicamente un dato informativo; el pago ocurre fuera de la blockchain. El registro tampoco prueba que el envase haya sido entregado físicamente: depende de la verificación del comercio. No se almacenan nombres, cédulas ni teléfonos.

## 3. ¿Por qué escogí este problema?

Porque es cotidiano en Costa Rica, especialmente en negocios pequeños que reutilizan envases, y puede resolverse sin crear un sistema grande de pagos o inventario. Permite practicar persistencia de datos, autorización de operaciones, validaciones y pruebas en Soroban.

## 4. Funciones del contrato

| Función | Qué hace | Quién puede ejecutarla |
|---|---|---|
| `inicializar(encargado)` | Establece el encargado, una sola vez | El encargado elegido debe autorizar |
| `registrar(id, cliente, monto_colones)` | Crea un depósito pendiente | Encargado |
| `devolver(id)` | Marca el depósito como devuelto | Encargado |
| `consultar(id)` | Devuelve los datos o `None` si no existe | Cualquier usuario |

Reglas: monto mayor a cero; IDs sin repetir; no se admite una segunda devolución; se exige autorización criptográfica de la dirección encargada para modificar registros. El estado se guarda en el almacenamiento `persistent` de Soroban, que tiene un tiempo de vida (TTL): para una aplicación real habría que gestionar la renovación de ese TTL.

## 5. Ejemplo simple

1. El negocio inicializa con su dirección Stellar.
2. Al prestar un envase, registra `id=1`, cliente `G...` y `monto_colones=500`.
3. `consultar(1)` muestra `devuelto=false`.
4. El cliente regresa el envase y el negocio le devuelve **₡500 por fuera de la blockchain**.
5. El encargado ejecuta `devolver(1)` y `consultar(1)` muestra `devuelto=true`.
6. Si intenta devolver el mismo depósito otra vez, el contrato rechaza la operación.

## 6. Cómo correrlo localmente

Requisitos: Rust/Cargo, target WASM de Stellar y Stellar CLI. Referencia oficial: https://developers.stellar.org/docs/build/smart-contracts/getting-started

Desde esta carpeta:

```bash
# Ejecuta las pruebas unitarias en el entorno Soroban del SDK
cargo test

# Compila el contrato para WASM (requiere Stellar CLI configurada)
stellar contract build
```

No se necesitan llaves privadas, una wallet real ni fondos para ejecutar las pruebas locales. Este proyecto no incluye un despliegue confirmado a Testnet.

## 7. Pruebas incluidas

En `src/test.rs` se definen cinco pruebas automatizadas:

1. Registrar un depósito, leer los datos y marcarlo devuelto.
2. Consultar un identificador inexistente y recibir `None`.
3. Rechazar un identificador repetido.
4. Rechazar una doble devolución.
5. Rechazar un monto de ₡0.

**Estado de verificación:** las pruebas están escritas, pero deben ejecutarse con `cargo test` en un entorno que tenga Rust y las dependencias instaladas. No se afirma que hayan pasado sin ejecutarlas. Para la tarea, conviene adjuntar una captura del resultado de `cargo test`.

## 8. Estructura

```text
stellar-envases-retornables/
├── Cargo.toml
├── .gitignore
├── README.md
└── src/
    ├── lib.rs   # Contrato Soroban
    └── test.rs  # Pruebas automatizadas
```

## 9. Posibles mejoras (fuera del alcance)

Agregar una interfaz para el negocio, emitir eventos, administrar el TTL de los registros, y utilizar una stablecoin para automatizar devoluciones. Ninguna es necesaria para demostrar este primer contrato.
