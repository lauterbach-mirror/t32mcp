# CI images (RUST_TEST_IMAGE, RUST_IMAGE and PYTEST_IMAGE in .gitlab-ci.yml),
# targets `rust-test`, `rust` and `pytest`. All contain a headless TRACE32
# RISC-V simulator, taken from a TRACE32 installation given as the `trace32`
# build context. `rust` is `rust-test` plus the cross toolchains.
#
# Building, pushing and updating: see ci/README.md. In short, the tags are
# <rust|python version>-t32-<T32_RELEASE>-r<n>, and RUST_VERSION must match
# t32mcp-rs/rust-toolchain.toml exactly, or rustup installs the pinned
# toolchain (without targets) in every job.

ARG RUST_VERSION=1.96.0
ARG PYTHON_VERSION=3.12
ARG UV_VERSION=0.13.0
# TRACE32 release of the `trace32` build context, recorded as an image label.
ARG T32_RELEASE=2026.02

# What the tests need from the TRACE32 installation: the simulator, help.t32
# (error message texts), per_notify.per (WINPRINT.PER) and t32pro.ps (PRINTER).
FROM scratch AS t32
COPY --from=trace32 bin/pc_linux64/t32mriscv /opt/t32/bin/pc_linux64/
COPY --from=trace32 help.t32 per_notify.per t32pro.ps /opt/t32/

# Lint and test only (clippy, test): no cross toolchains, about half the size
# of `rust`, so these jobs pull faster.
FROM rust:${RUST_VERSION}-slim-bookworm AS rust-test

ARG T32_RELEASE
LABEL com.lauterbach.trace32.release=${T32_RELEASE}

# cargo-nextest: JUnit test reports; libcurl: TRACE32.
RUN set -eux; \
    rustup component add clippy rustfmt; \
    apt-get update; \
    apt-get install -y --no-install-recommends libcurl4; \
    rm -rf /var/lib/apt/lists/*; \
    cargo install --locked cargo-nextest@0.9.148; \
    rm -rf "$CARGO_HOME/registry"

COPY --from=t32 /opt/t32 /opt/t32
ENV T32SYS=/opt/t32
# Fail the build, not the tests, if the simulator misses a library.
RUN ! ldd "$T32SYS/bin/pc_linux64/t32mriscv" | grep "not found"

# Cross checks and release builds (check:cross, build:*).
FROM rust-test AS rust

ARG ZIG_VERSION=0.17.0

# aarch64 gcc: Linux ARM64 linker; cargo-xwin: Windows; zig + cargo-zigbuild:
# macOS (zig ships the SDK stubs and linker).
RUN set -eux; \
    rustup target add \
        aarch64-apple-darwin \
        aarch64-unknown-linux-gnu \
        x86_64-apple-darwin \
        x86_64-pc-windows-msvc; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        gcc-aarch64-linux-gnu \
        libc6-dev-arm64-cross \
        wget \
        xz-utils; \
    rm -rf /var/lib/apt/lists/*; \
    cargo install --locked cargo-xwin@0.23.1; \
    cargo install --locked cargo-zigbuild@0.24.0; \
    rm -rf "$CARGO_HOME/registry"; \
    wget -q "https://ziglang.org/download/${ZIG_VERSION}/zig-x86_64-linux-${ZIG_VERSION}.tar.xz"; \
    echo "1cbe9df9f27e6b78d14ccbca43b6703a404ef79ef1c463de901d7f088d4e2026  zig-x86_64-linux-${ZIG_VERSION}.tar.xz" | sha256sum -c -; \
    tar -xJf "zig-x86_64-linux-${ZIG_VERSION}.tar.xz" -C /opt; \
    ln -s "/opt/zig-x86_64-linux-${ZIG_VERSION}/zig" /usr/local/bin/zig; \
    rm "zig-x86_64-linux-${ZIG_VERSION}.tar.xz"

FROM ghcr.io/astral-sh/uv:${UV_VERSION}-python${PYTHON_VERSION}-trixie-slim AS pytest

ARG T32_RELEASE
LABEL com.lauterbach.trace32.release=${T32_RELEASE}

# libcurl: TRACE32.
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends libcurl4t64; \
    rm -rf /var/lib/apt/lists/*

COPY --from=t32 /opt/t32 /opt/t32
ENV T32SYS=/opt/t32
RUN ! ldd "$T32SYS/bin/pc_linux64/t32mriscv" | grep "not found"
