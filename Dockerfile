### Build arguments
ARG APP_NAME=snmp_sim
ARG BASE_IMAGE=debian:10-slim
ARG BUILDER_IMAGE=lukemathwalker/cargo-chef:latest-rust-1.62-buster
ARG CARGO_REGISTRY_URL=
ARG RELEASE_OR_DEBUG=release

FROM ${BUILDER_IMAGE} as custom_builder
ARG CARGO_REGISTRY_URL

# Install git - debian
RUN apt update && apt install -y python3 python3-pip git jq pkg-config libssl-dev && git config --global credential.helper store && echo "${CARGO_REGISTRY_URL}" > ~/.git-credentials

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

### Analyze the current project to determine the minimum subset of files (Cargo.lock
### and Cargo.toml manifests) required to build it and cache dependencies
FROM custom_builder as planner
ARG APP_NAME

WORKDIR /${APP_NAME}/

COPY . .
COPY .cargo/config .cargo/config

RUN cargo chef prepare  --recipe-path recipe.json

### Re-hydrate the minimum project identified by `cargo chef prepare` and
### build it to cache dependencies
FROM custom_builder as builder
ARG APP_NAME
ARG RELEASE_OR_DEBUG

WORKDIR /${APP_NAME}/

COPY --from=planner /${APP_NAME}/recipe.json recipe.json
COPY .cargo/config .cargo/config

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin "${APP_NAME}"

### Minimal runtime image
FROM ${BASE_IMAGE} as runtime
ARG RELEASE_OR_DEBUG
ARG APP_NAME

COPY --from=builder /${APP_NAME}/target/${RELEASE_OR_DEBUG}/${APP_NAME} /service
COPY --from=builder /${APP_NAME}/configuration/base.yaml /configuration/base.yaml
COPY --from=builder /lib/x86_64-linux-gnu/libm* /lib/x86_64-linux-gnu/

ENTRYPOINT ["/service"]
