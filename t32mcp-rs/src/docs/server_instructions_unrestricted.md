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
4. trace32_run_command: last resort, for a single command that no script covers, or to
   operate the dialog of a paused script. Look the command up first, e.g. with
   the t32chat MCP server (trace32_search_docs, trace32_check_syntax). Do not
   guess commands.
Do not drive the debugger by other means (Python, Remote API code, shell
scripts, GUI automation) unless the user asks for it.