# STAGE: BUILDER (modded from https://github.com/rust-lang/docker-rust-nightly/blob/master/debian/Dockerfile for compatibility)

FROM buildpack-deps:bookworm AS builder

ARG DISABLE_UPX

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    \
    url="https://static.rust-lang.org/rustup/dist/${rustArch}/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME;

# Added for compatibility with custom script
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

WORKDIR /builder

COPY . /builder

RUN sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b /usr/local/bin

RUN task build:release-opt

RUN if [ -z "$DISABLE_UPX" ] || [ "$DISABLE_UPX" = "false" ] || [ "$DISABLE_UPX" = "0" ]; then \
      # apt-get update && apt-get install -y wget xz-utils && \ # <- wget and xz-utils are preinstalled \
      wget https://github.com/upx/upx/releases/download/v5.0.0/upx-5.0.0-amd64_linux.tar.xz && \
      tar -xf upx-5.0.0-amd64_linux.tar.xz && \
      mv upx-5.0.0-amd64_linux/upx /usr/local/bin/upx && \
      chmod +x /usr/local/bin/upx && \
      rm -rf upx-5.0.0-amd64_linux.tar.xz upx-5.0.0-amd64_linux && \
      upx --best --ultra-brute /builder/target/release/plist-server; \
    fi

# RUNTIME

FROM bitnami/minideb:latest AS runtime

ENV \
    PORT=8080 \
    HOST=0.0.0.0

WORKDIR /app

RUN install_packages ca-certificates curl && \
    rm -rf /var/cache && \
    rm -rf /var/lib/apt/extended_states /var/lib/apt/lists/* && \
    rm -rf /var/log/* && \
    rm -rf /var/lib/dpkg/status /var/lib/dpkg/status-old;

COPY --from=builder /builder/src/target/release/plist-server /app/plist-server

EXPOSE 8080

ENTRYPOINT ["/app/plist-server"]
