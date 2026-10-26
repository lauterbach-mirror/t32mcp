# t32mcp Tests

The tests need to run sequentially in order to be able to use the Remote API exclusively:

```bash
$ cargo test -- --test-threads=1
```

`cargo nextest run` (as in CI) serializes them by itself, see `.config/nextest.toml`.

They need a PowerView on `T32MCP_T32_PORT` (default: 20000). With `T32SYS` set to a TRACE32 installation, `ci/start_sim.sh` starts a headless RISC-V Instruction Set Simulator for them (`tests/sim.t32`); the CI images contain one. `ci/test_rust.sh` (also the pre-commit hook) does all of it: it starts the simulator if needed, runs the tests and stops it again.

Some of the tests are quite specific (e.g. `flood_pipe`) and therefore take a long time to execute.
Per default, they are ignored but can be executed by passing the `--ignored` flag to `cargo test --`.

When testing `flood_pipe` on Windows, run with release mode to prevent a pipe overflow:

```bash
cargo test -r flood_pipe -- --test-threads=1 --nocapture --ignored
```

