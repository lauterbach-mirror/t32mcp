Run PRACTICE commands in TRACE32 PowerView and return their output.

Last resort. Use trace32_run_script with a skill script whenever one
covers the task: skill scripts document their arguments and output. If no
script fits and the task needs several commands or comes up again, write a
script (skill lauterbach-trace32-skill-creator).
Use trace32_run_command only for a single command that no script covers, or to operate
the dialog of a paused script. Look the command up before running it, e.g. with
the t32chat MCP server (trace32_search_docs, trace32_check_syntax). Do not
guess commands. Still prefer it over writing Remote API code (e.g. Python).
PRACTICE can change the target and run host commands.

Only output printed to the AREA window is returned:
    - PRINT <expression>, e.g. PRINT Register(PC)
    - WinPrint.<window command>, e.g. WinPrint.Register.view
A window command alone (e.g. Register.view) opens a window and returns nothing.

The command, or a multi-line command sequence, runs as a temporary script.
If a script is still active (running after a timeout, or paused in a dialog),
the call fails with its state and call stack, unless if_busy says otherwise.
With if_busy=nest, the commands can operate the dialog of a paused script:
DIALOG.SELect <name>, DIALOG.Set <label> <value>, DIALOG.EXecute <label>,
PRINT DIALOG.STRing(<label>). CONTinue resumes the paused script.
If a command fails, the call fails with the error message, the failing command
and the output so far.

If collect_output is false, a single command line is submitted without waiting
for its output (fire-and-forget).

port selects the PowerView instance, core the core within it (CORE.Select
first). Without port, the first reachable instance is used; without core, its
selected core. trace32_get_status lists the ports and cores.

The connection to TRACE32 is established lazily and re-established automatically,
so a failed call can be retried after launching PowerView.

Possible errors:
    - TRACE32 PowerView is not running or the API Port is not enabled
    - A command failed
    - Timeout: the command waits, e.g. in WAIT or a modal dialog (Windows)
    - PowerView does not respond: busy, hangs, or a modal dialog (Linux)
    - A multi-line command was given with collect_output set to false