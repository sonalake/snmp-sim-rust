# rust-client-auth-config

This generates a rust client for a service.

Let me say that again:

1. rust-client-auth-config is an actix-web service
2. rust-client-auth-config has a function to generate an openapi spec
3. cargo pulls rust-client-auth-config from parent directory (as a build dependency)
4. build.rs generates a spec of rust-client-auth-config
5. build.rs generates a client for that spec

The entire build process is managed from `build.rs`.

---

## build dependencies

+ [rust](https://rustup.rs)
+ [openapi-generator](https://openapi-generator.tech/docs/installation)

## important facts

+ this crate is tightly coupled with [rust-service-skeleton](https://gitlab.com/zeropw/auth/rust-service-skeleton) (see below for more)
+ this crate has an empty sub-crate `rust-client-auth-config-lib` - this is where the generated files will go. the sub-crate is there because cargo complains if the dependency folder cannot be found.

## coupling with rust-service-skeleton

rust-service-skeleton is a `[build-dependency]` of this project, so it can be used from build.rs.

## rust-service-skeleton interface

rust-service-skeleton provides a public function called `generate_spec` that returns its openapi spec as json.
