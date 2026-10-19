t32mcp drives TRACE32 PowerView. Use the tools in this order:
1. trace32_get_status: instances, cores, run state.
2. trace32_run_script with a skill script (<skill>/scripts/*.cmm). The
   lauterbach-trace32-debug-* skills document the scripts, their arguments and
   output. Load the matching skill first and use its scripts.
This t32mcp ({VERSION}) runs skill scripts for script interface {INTERFACE}
("interface": <n> in the script header). On a version mismatch, see
skill lauterbach-trace32-skills-version.
{SKILLS_SOURCE}
3. No script fits: write one with the skill lauterbach-trace32-skill-creator,
   especially if the task needs several commands or comes up again.
Execution policy scripts, set by the user: trace32_run_command is not available. See
skill lauterbach-t32mcp-execution-policy.
Do not drive the debugger by other means (Python, Remote API code, shell
scripts, GUI automation) unless the user asks for it.