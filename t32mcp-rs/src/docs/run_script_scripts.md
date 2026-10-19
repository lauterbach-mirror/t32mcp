Execute a PRACTICE script (*.cmm) in TRACE32 PowerView and return its output.

The only tool to act on the target: execution policy scripts, set by the user
(no trace32_run_command). See skill lauterbach-t32mcp-execution-policy.
Use it with the scripts of the loaded skills (<skill>/scripts/*.cmm): their
SKILL.md documents arguments and output.
script_path is an absolute path, or starts with ~~ (TRACE32 system directory).
Other scripts (e.g. demo scripts) often return nothing and may block in
dialogs; see skill lauterbach-trace32-skill-creator.
PRACTICE can run host commands and change the target (e.g. erase flash). Ask the
user before running a script that is not part of a skill.

{SKILLS_SOURCE}

The script header (the first JSON object in the script's leading comment block:
"interface", "strict", "arguments") is checked: the interface
version and script_args against the declared arguments: missing required
arguments (an argument is required unless it declares "optional": true),
unknown arguments,
enum and pattern. Argument problems reject the call under every policy, other header problems are warnings before the output, the script runs
anyway (it may fail). The message
names the argument and what it must match. Fix the argument, do not try to get
around the pattern: it keeps arguments from adding options or commands to the
script's command lines.
This t32mcp ({VERSION}) supports script interface {INTERFACE}. On the
PowerView host: t32mcp --version.

Only output printed to the selected AREA window is returned
(PRINT, or PRinTer.Area followed by WinPrint.<command>).

port selects the PowerView instance, core the core within it (CORE.Select
before the script runs, the original core is selected again afterwards).
Without port, the first reachable instance is used; without core, its
selected core. trace32_get_status lists the ports and cores.

This tool waits for the script to finish.
If a script is still active (running after a timeout, or paused in a dialog),
the call fails with its state and call stack, unless if_busy says otherwise.
If the script stops at an error, it is terminated and the call fails with the
error message, the failing command, its location and the output so far.
If the script pauses at STOP, e.g. waiting in a dialog, it keeps waiting. The
result shows the call stack and the open dialogs. Ask the user to operate them.
If the script times out, it keeps running. The result shows where it is.
Modal dialogs (DIALOG.OK, DIALOG.YESNO, DIALOG.File) have no name. The script waits in them until the user answers. On
Windows, the call times out and terminating the script leaves the dialog open.
On Linux, they block the Remote API: every call fails with "PowerView does not
respond" until the dialog is closed. Ask the user to answer it.
The connection to TRACE32 is established lazily and re-established automatically,
so a failed call can be retried after launching PowerView.

Possible errors:
    - script_path is not a *.cmm file, not absolute or not found
    - An argument does not match its declaration in the script header
    - TRACE32 PowerView is not running or the API Port is not enabled
    - The script stopped at an error
    - Timeout: the script waits, e.g. in WAIT or a modal dialog (Windows)
    - PowerView does not respond: busy, hangs, or a modal dialog (Linux)