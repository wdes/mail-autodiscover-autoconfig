FROM rust:1.61.0-alpine3.16 as build-env

WORKDIR /workspace
COPY . ./
RUN apk add --no-cache --update musl-dev openssl-dev
# https://users.rust-lang.org/t/sigsegv-with-program-linked-against-openssl-in-an-alpine-container/52172
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release

FROM alpine:3.16 as runtime

ARG VCS_REF
ARG BUILD_DATE

# Metadata
LABEL org.label-schema.vendor="mail-autodiscover-autoconfig" \
    org.label-schema.url="https://github.com/wdes/mail-autodiscover-autoconfig#readme" \
    org.label-schema.name="mail-autodiscover-autoconfig" \
    org.label-schema.description="mail-autodiscover-autoconfig server" \
    org.label-schema.version=${VERSION} \
    org.label-schema.vcs-url="https://github.com/wdes/mail-autodiscover-autoconfig.git" \
    org.label-schema.vcs-ref=${VCS_REF} \
    org.label-schema.build-date=${BUILD_DATE} \
    org.label-schema.docker.schema-version="1.0"

# https://github.com/mischov/meeseeks/issues/98#issuecomment-636615680
RUN apk add --no-cache --update openssl libgcc

ENV ROCKET_TEMPLATE_DIR="/usr/lib/mail-autodiscover-autoconfig/templates"
COPY --from=build-env /workspace/templates /usr/lib/mail-autodiscover-autoconfig/templates
COPY --from=build-env /workspace/target/release/mail-autodiscover-autoconfig /usr/bin

ENTRYPOINT [ "mail-autodiscover-autoconfig" ]
