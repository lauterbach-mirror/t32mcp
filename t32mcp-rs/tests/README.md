# t32mcp Tests

The tests need to run sequentially in order to be able to use the Remote API exclusively:

```bash
$ cargo test -- --test-threads=1
```

Some of the tests are quite specific (e.g. `flood_pipe`) and therefore take a long time to execute.
Per default, they are ignored but can be executed by passing the `--ignored` flag to `cargo test --`.

When testing `flood_pipe` on Windows, run with release mode to prevent a pipe overflow:

```bash
cargo test -r flood_pipe -- --test-threads=1 --nocapture --ignored
```

