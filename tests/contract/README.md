# Script interface contract tests

t32mcp supports a single script interface version (`SCRIPT_INTERFACE_VERSION`
in `t32mcp-rs/src/execution_policy.rs`). Skill scripts declare theirs with `"interface": <n>`
in the header JSON object.

`interface-<n>/contract/` is a skill with fixture scripts written against interface `<n>`, the way the
skill scripts of that time use it: arguments via `ENTRY %LINE` and `STRing.SCANAndExtract`, output via
the AREA, errors via `ON ERROR`.

- Never edit the fixtures of a released interface version. A t32mcp change that makes them fail
  breaks the scripts of that version: bump `SCRIPT_INTERFACE_VERSION`, add `interface-<n+1>/`,
  and bump the major version of the skills.
- Run: `uv run pytest test_contract.py` (in `tests/`) with a PowerView (e.g. the Instruction Set Simulator) on
  `T32MCP_T32_PORT` (default 20000). Skipped if none is reachable.
