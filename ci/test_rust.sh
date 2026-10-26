#!/usr/bin/env bash
# Run the Rust tests for the pre-commit hook, like the CI `test` job: the unit
# tests, and with a PowerView also the tests in t32mcp-rs/tests/. If none
# listens on T32MCP_T32_PORT (default: 20000) and T32SYS is set, starts the
# simulator (start_sim.sh) for the run and stops it afterwards. Without either,
# only the unit tests run.
set -eu

export T32MCP_T32_PORT="${T32MCP_T32_PORT:-20000}"
cd "$(dirname "$0")/.."

listening() {
  (echo > "/dev/tcp/127.0.0.1/$T32MCP_T32_PORT") 2> /dev/null
}

# Stop the simulator started below and wait until it is gone, so that the next
# run (e.g. the pytest hook) does not find it still listening.
stop_sim() {
  local pid
  pid="$(cat "$pidfile" 2> /dev/null)" || pid=
  rm -f "$pidfile"
  [ -n "$pid" ] || return 0
  kill "$pid" 2> /dev/null || return 0
  for _ in $(seq 50); do
    kill -0 "$pid" 2> /dev/null || return 0
    sleep 0.1
  done
  kill -9 "$pid" 2> /dev/null || true
}

if ! listening; then
  if [ -z "${T32SYS:-}" ]; then
    echo "No PowerView on port $T32MCP_T32_PORT and no T32SYS to start one: unit tests only." >&2
    exec cargo test --manifest-path t32mcp-rs/Cargo.toml --lib --bins
  fi
  pidfile="$(mktemp)"
  trap stop_sim EXIT
  ci/start_sim.sh "$pidfile"
fi

# One PowerView runs one script at a time, see t32mcp-rs/tests/README.md.
cargo test --manifest-path t32mcp-rs/Cargo.toml -- --test-threads=1
