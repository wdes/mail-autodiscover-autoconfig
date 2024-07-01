FROM alpine:3 as build-env

WORKDIR /workspace
COPY . ./
RUN apk add --no-cache --update musl-dev openssl-dev rust cargo
RUN cargo test --release
# https://users.rust-lang.org/t/sigsegv-with-program-linked-against-openssl-in-an-alpine-container/52172
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release

FROM alpine:3 as runtime

ARG VCS_REF
ARG BUILD_DATE
ARG RELEASE_VERSION

# Metadata
LABEL org.label-schema.vendor="Wdes" \
    org.label-schema.url="https://github.com/wdes/mail-autodiscover-autoconfig#readme" \
    org.label-schema.name="mail-autodiscover-autoconfig" \
    org.label-schema.description="A docker image to deploy to have a dynamic autodiscover XML and autoconfig XML host. Supports Outlook, Thunderbird, Apple, some Android apps, and more.." \
    org.label-schema.version=${RELEASE_VERSION} \
    org.label-schema.vcs-url="https://github.com/wdes/mail-autodiscover-autoconfig.git" \
    org.label-schema.vcs-ref=${VCS_REF} \
    org.label-schema.build-date=${BUILD_DATE} \
    org.label-schema.docker.schema-version="1.0" \
    \
    com.docker.extension.publisher-url="https://github.com/wdes" \
    \
    org.opencontainers.image.title="Docker autodiscover, autoconfig web server" \
    org.opencontainers.image.description="A docker image to deploy to have a dynamic autodiscover XML and autoconfig XML host. Supports Outlook, Thunderbird, Apple, some Android apps, and more.." \
    org.opencontainers.image.authors="williamdes@wdes.fr" \
    org.opencontainers.image.url="https://github.com/wdes/mail-autodiscover-autoconfig#readme" \
    org.opencontainers.image.documentation="https://github.com/wdes/mail-autodiscover-autoconfig#readme" \
    org.opencontainers.image.source="https://github.com/wdes/mail-autodiscover-autoconfig" \
    org.opencontainers.image.vendor="Wdes" \
    org.opencontainers.image.licenses="MPL-2.0" \
    org.opencontainers.image.created=${BUILD_DATE} \
    org.opencontainers.image.version=${RELEASE_VERSION} \
    org.opencontainers.image.revision=${VCS_REF} \
    org.opencontainers.image.ref.name="latest"

# https://github.com/mischov/meeseeks/issues/98#issuecomment-636615680
RUN apk add --no-cache --update openssl libgcc

ENV ROCKET_TEMPLATE_DIR="/usr/lib/mail-autodiscover-autoconfig/templates"
COPY --from=build-env /workspace/templates /usr/lib/mail-autodiscover-autoconfig/templates
COPY --from=build-env /workspace/target/release/mail-autodiscover-autoconfig /usr/bin

ENTRYPOINT [ "mail-autodiscover-autoconfig" ]
