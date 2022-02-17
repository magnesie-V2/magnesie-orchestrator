# Builder
FROM rust:1.58.1 as builder

RUN USER=root cargo new --bin magnesie-orchestrator
WORKDIR ./magnesie-orchestrator
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/main*
RUN cargo build --release




FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata netcat \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 7878

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /magnesie-orchestrator/target/release/main ${APP}/magnesie-orchestrator

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER

COPY wait-for.sh /bin/

WORKDIR ${APP}

