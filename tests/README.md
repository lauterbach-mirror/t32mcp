# t32mcp Integration Tests

**NOTE:** Local debug build of `t32mcp-rs` required to run tests.

```bash
uv run pytest
```

Environment variables:
- `T32MCP`: path to the `t32mcp` executable (default: the local debug build).
- `T32MCP_HTTP_PORT`: port for the HTTP server tests (default: a free port).
- `T32MCP_T32_PORT`: Remote API port of a PowerView for the contract tests (default: 20000). Without a reachable PowerView they are skipped.

`test_contract.py`: contract tests of the script interface, see `contract/README.md`.
