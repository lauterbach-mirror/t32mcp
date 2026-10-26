#!/usr/bin/env bash
# Start the headless TRACE32 RISC-V simulator (tests/sim.t32) in the background
# and wait until its Remote API accepts connections. For the tests in
# t32mcp-rs/tests/. The Python tests in tests/ start it themselves.
#
# Usage: ci/start_sim.sh [pidfile]
#   Needs T32SYS. T32MCP_T32_PORT defaults to 20000. Writes the PID of
#   PowerView to pidfile, if given, so the caller can stop it again.
#   Linux (x86_64, aarch64), macOS and Windows (Git Bash).
set -eu

# The RISC-V simulator of the TRACE32 installation for this platform. Keep in
# sync with powerview_executable() in tests/conftest.py.
case "$(uname -s)-$(uname -m)" in
  Linux-aarch64) exe=bin/linux-aarch64/t32mriscv ;;
  Linux-*) exe=bin/pc_linux64/t32mriscv ;;
  Darwin-*) exe=bin/macosx64/t32mriscv-qt ;;
  MINGW*|MSYS*|CYGWIN*) exe=bin/windows64/t32mriscv.exe ;;
  *)
    echo "ERROR: no TRACE32 simulator known for $(uname -s) $(uname -m)." >&2
    exit 1
    ;;
esac

export T32MCP_T32_PORT="${T32MCP_T32_PORT:-20000}"
cd "$(dirname "$0")/.."

# pwerview output to a log file, not /dev/null, to check in case of crashes
log="${TMPDIR:-/tmp}/t32mcp-sim-$T32MCP_T32_PORT.log"
"$T32SYS/$exe" -c tests/sim.t32 > "$log" 2>&1 &
pid=$!
if [ $# -gt 0 ]; then
  echo "$pid" > "$1"
fi

for _ in $(seq 150); do
  if (echo > "/dev/tcp/127.0.0.1/$T32MCP_T32_PORT") 2> /dev/null; then
    echo "PowerView (pid $pid) listens on port $T32MCP_T32_PORT."
    exit 0
  fi
  if ! kill -0 "$pid" 2> /dev/null; then
    echo "ERROR: PowerView exited before it listened on port $T32MCP_T32_PORT. Its output ($log):" >&2
    cat "$log" >&2
    exit 1
  fi
  sleep 0.2
done
echo "ERROR: PowerView does not listen on port $T32MCP_T32_PORT after 30 s. Its output ($log):" >&2
cat "$log" >&2
exit 1
