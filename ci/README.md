# CI

Maintainer notes for the GitLab pipeline (`.gitlab-ci.yml`) and the files in this directory.
Contributors don't need this: [CONTRIBUTING.md](../CONTRIBUTING.md) covers the local hooks.

| File | Purpose |
|---|---|
| `Dockerfile` | The three CI images, see [Images](#images). |
| `start_sim.sh` | Starts the TRACE32 simulator for the Rust tests and waits for its port. |
| `test_rust.sh` | Runs the Rust tests for the pre-commit hook, with the simulator if possible. |
| `check_version.sh` | Release check: the version matches `t32mcp-rs/Cargo.toml`. |
| `extract_changelog.sh` | Release notes: the `## [<version>]` section of `CHANGELOG.md`. |

## Pipelines

**Push** (no inputs set), all in the `test` stage, in parallel:

| Job | Image | What it does |
|---|---|---|
| `clippy` | `RUST_TEST_IMAGE` | `cargo clippy -D warnings` |
| `test` | `RUST_TEST_IMAGE` | builds the debug binary, runs all Rust tests (unit tests and `t32mcp-rs/tests/` against the simulator) with cargo-nextest |
| `test:integration` | `PYTEST_IMAGE` | runs the Python tests in `tests/` against that binary and the simulator |
| `check:cross` | `RUST_IMAGE` | `cargo check` for the release targets that aren't built on push |
| `sast`, `secret_detection` | GitLab's | security scans from the included templates |

`test` and `test:integration` upload JUnit reports, shown in the pipeline's **Tests** tab and in the
merge request.

**Release** (`release_version` input set): SAST, secret detection, clippy and the tests are skipped
(they ran on the push that landed the commit). The pipeline runs `verify:release`, the five `build:*`
jobs and `package:skill`, then:
- on the default branch: `upload` (Generic Package Registry) and `release` (tag + GitLab Release)
- with `release_dry_run`: only `release:dry-run`, which checks the files and the release notes.

Tag pipelines are disabled, so the tag that `release` creates doesn't start another pipeline.

## Caches

Every Rust job has its own cache with the cargo registry (`index/`, `cache/`) and `t32mcp-rs/target/`.
`build:windows` also has a second one for `.xwin-cache/` (the Windows CRT/SDK that cargo-xwin downloads).

| Cache | Key | Starts fresh when |
|---|---|---|
| `cargo-<job>-<hash>` | `Cargo.lock`, `rust-toolchain.toml` | a dependency or the Rust version changes |
| `xwin-<hash>` | `ci/Dockerfile` | the image changes (it pins cargo-xwin) |

So a new cache never contains stale artifacts of old dependencies or toolchains. `CARGO_INCREMENTAL=0`
and `CARGO_PROFILE_DEV_DEBUG=line-tables-only` keep the debug target dirs about half as big.

Only pipelines on the default branch save the caches (`CARGO_CACHE_POLICY: pull-push`). All other
branches only restore them. So after a change to one of the key files, a branch builds without a
cache until it is merged.

## Releasing

1. Bump `version` in `t32mcp-rs/Cargo.toml` and add a `## [<version>] - <date>` section to
   `CHANGELOG.md` and merge to the default branch.
2. Optional: try the release pipeline on any branch first, without publishing anything:
   ```sh
   glab ci run -b <branch> --input release_version:v0.4.0 --input release_dry_run:true
   ```
3. Release from the default branch:
   ```sh
   glab ci run -b main --input release_version:v0.4.0
   ```
   Or use **Build → Pipelines → Run pipeline** and fill in the inputs. `release_version` must look
   like `v1.2.3` (pre-release suffixes such as `-rc1` are allowed). If left empty, no release job is
   created.

## Images

All images are built from `Dockerfile` and pushed to this project's container registry
(project members only). Their tags are in `.gitlab-ci.yml` (`RUST_TEST_IMAGE`, `RUST_IMAGE`,
`PYTEST_IMAGE`):

| Image | Target | Base | Used by |
|---|---|---|---|
| `ci/rust-test:<rust>-t32-<trace32>-r<n>` | `rust-test` | `rust:<rust>-slim-bookworm` | clippy, test |
| `ci/rust:<rust>-t32-<trace32>-r<n>` | `rust` | `rust-test` | check:cross, build:* |
| `ci/pytest:<python>-t32-<trace32>-r<n>` | `pytest` | `ghcr.io/astral-sh/uv:<uv>-python<python>-trixie-slim` | test:integration |

The `rust-test` image contains clippy, rustfmt and cargo-nextest. The `rust` image adds the release
targets, the arm64 cross gcc, cargo-xwin (Windows) and zig + cargo-zigbuild (macOS), so the jobs
download nothing. Jobs on GitLab's shared runners pull the whole image every time and the pull counts
toward the compute minutes, so `clippy` and `test` use `rust-test`, which is half the size. The `pytest`
image is based on trixie because uv publishes versioned images only for trixie (the floating
`python3.12-bookworm-slim` tag no longer gets updates).

### TRACE32 simulator

All images contain a (headless) TRACE32 RISC-V Instruction Set Simulator in `/opt/t32` (`T32SYS`) for
the tests that need PowerView. From the local TRACE32 installation, only these files are copied:

| File | Needed for |
|---|---|
| `bin/pc_linux64/t32mriscv` | the simulator (needs libcurl, installed in all images) |
| `help.t32` | error message texts, without it PowerView reports only IDs (Rust test `error`) |
| `per_notify.per` | `WINPRINT.PER` (Rust test `winprint`) |
| `t32pro.ps` | `PRINTER`, otherwise a warning |

The TRACE32 installation location is given at build time as the `trace32` build context and should
point to the latest official DVD release.

The simulator is configured by `tests/sim.t32` (`SYS` and `PORT` from `T32SYS` and
`T32MCP_T32_PORT`). In `test`, `start_sim.sh` starts it, in `test:integration` the pytest fixture
does (see `tests/README.md`). Both work the same way on a developer machine with `T32SYS` set, and
pick the simulator of the platform: `bin/pc_linux64/t32mriscv`, `bin/linux-aarch64/t32mriscv`,
`bin/macosx64/t32mriscv-qt` or `bin/windows64/t32mriscv.exe` (the shell scripts need Git Bash on
Windows). They write PowerView's output to a log file.

### Building and pushing (with podman)

```sh
T32=/path/to/T32  # TRACE32 installation (DVD release)
REG=registry.gitlab.com/lauterbach/t32mcp/ci

podman build --build-context trace32=$T32 --target rust-test -t $REG/rust-test:1.96.0-t32-2026.02-r1 -f ci/Dockerfile ci
podman build --build-context trace32=$T32 --target rust -t $REG/rust:1.96.0-t32-2026.02-r2 -f ci/Dockerfile ci
podman build --build-context trace32=$T32 --target pytest -t $REG/pytest:3.12-t32-2026.02-r1 -f ci/Dockerfile ci

podman login registry.gitlab.com  # token with write access to the container registry
podman push $REG/rust-test:1.96.0-t32-2026.02-r1
podman push $REG/rust:1.96.0-t32-2026.02-r2
podman push $REG/pytest:3.12-t32-2026.02-r1
```

Push the images before the commit that references them, or the jobs fail to pull.
Never overwrite a pushed tag: bump `r<n>` instead, so older pipelines keep their image.

### Updating

Every image change follows the same steps: edit `Dockerfile`, set the new tags in the build commands
above and in `.gitlab-ci.yml`, build, [test locally](#testing-images-locally), push the images,
then push the commit.

| Change | Edit | New tags |
|---|---|---|
| Rust | `RUST_VERSION` and `t32mcp-rs/rust-toolchain.toml` (must match exactly, or rustup installs the toolchain without targets in every job) | `ci/rust-test` and `ci/rust`: `<new rust>-t32-...-r1` |
| TRACE32 | `T32_RELEASE`, and build from the new installation | all: `...-t32-<new release>-r1` |
| Python or uv | `PYTHON_VERSION`, `UV_VERSION` (check that the `<uv>-python<python>-trixie-slim` tag exists) | `ci/pytest:<python>-t32-...-r1` |
| Tools (cargo-xwin, zig, nextest, ...) or packages | the `RUN` lines | bump `r<n>` of the changed image; `ci/rust` is built on `rust-test`, so a change there bumps both |

Keep the `T32_RELEASE` label and the tags in sync.
`podman inspect --format '{{index .Labels "com.lauterbach.trace32.release"}}' <image>`
shows which release an image contains.

### Testing images locally

Run the jobs in a clean copy of the committed state, with the network cut off for the tests:

```sh
mkdir /tmp/ci-job && git archive HEAD | tar -x -C /tmp/ci-job && cd /tmp/ci-job
J="-v $PWD:/builds/p -w /builds/p -e CARGO_HOME=/builds/p/.cargo"

# test: fetch the crates (as the CI cache would), then run offline
podman run --rm $J $REG/rust-test:<tag> sh -c 'cd t32mcp-rs && cargo fetch'
podman run --rm --network none $J -e T32MCP_T32_PORT=20000 -e CARGO_NET_OFFLINE=true $REG/rust-test:<tag> bash -c 'ci/start_sim.sh && cd t32mcp-rs && cargo build && cargo nextest run --profile ci'

# test:integration: uses the binary built above
podman run --rm -w /builds/p/tests -v $PWD:/builds/p $REG/pytest:<tag> uv sync --frozen
podman run --rm --network none -w /builds/p/tests -v $PWD:/builds/p $REG/pytest:<tag> uv run --offline pytest
```

All tests must pass and none may be skipped.

## Pre-commit hooks

`.pre-commit-config.yaml` runs gitleaks on the staged changes (secret detection before anything is
pushed), then, for commits that touch Rust files, rustfmt, clippy, build, the Rust tests (`test_rust.sh`)
and the Python tests. Install: see [CONTRIBUTING.md](../CONTRIBUTING.md).

With `T32SYS` set, both test hooks start the simulator themselves (and stop it again), so all tests
run as in CI. Without it and without a PowerView on `T32MCP_T32_PORT`, the Rust hook runs only the
unit tests and the Python tests that need a PowerView are skipped.
