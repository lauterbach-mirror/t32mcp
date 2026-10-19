t32mcp drives TRACE32 PowerView. Use the tools in this order:
1. trace32_get_status: instances, cores, run state.
2. trace32_run_script with a skill script (<skill>/scripts/*.cmm). The
   lauterbach-trace32-debug-* skills document the scripts, their arguments and
   output. Load the matching skill first and use its scripts.
This t32mcp ({VERSION}) runs skill scripts for script interface {INTERFACE}
("interface": <n> in the script header). On a version mismatch, see
skill lauterbach-trace32-skills-version.
{SKILLS_SOURCE}
Execution policy strict, set by the user: only skill scripts that declare
"strict": true in their header run, trace32_run_command is not available.
No script fits: tell the user and stop. Do not write new scripts or skills, do not
change existing ones, and do not drive the debugger by other means (Python,
Remote API, t32rem). See skill lauterbach-t32mcp-execution-policy.