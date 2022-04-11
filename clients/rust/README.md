# rust-client-snmp-sim

This generates a rust client for a service.

Let me say that again:

1. rust-client-snmp-sim is an actix-web service
2. rust-client-snmp-sim has a function to generate an openapi spec
3. cargo pulls rust-client-snmp-sim from parent directory (as a build dependency)
4. build.rs generates a spec of rust-client-snmp-sim
5. build.rs generates a client for that spec

The entire build process is managed from `build.rs`.

---

## build dependencies

+ [rust](https://rustup.rs)
+ [openapi-generator](https://openapi-generator.tech/docs/installation)

## important facts

+ this crate is tightly coupled with [snmp-sim-rust](https://github.com/sonalake/snmp-sim-rust) (see below for more)
+ this crate has an empty sub-crate `rust-client-snmp-sim-lib` - this is where the generated files will go. the sub-crate is there because cargo complains if the dependency folder cannot be found.

## coupling with snmp-sim-rust

snmp-sim-rust is a `[build-dependency]` of this project, so it can be used from build.rs.

## snmp-sim-rust interface

snmp-sim-rust provides a public function called `generate_spec` that returns its openapi spec as json.
