Execute a PRACTICE script (*.cmm) in TRACE32 PowerView and return its output.

The only tool to act on the target: execution policy strict, set by the
user. Only skill scripts run (<skill>/scripts/*.cmm next to <skill>/SKILL.md,
inside the --skills directories) that declare "strict": true in their header.
Other scripts are rejected. No trace32_run_command. Each skill's SKILL.md
documents the arguments and output of its scripts. If no skill script fits,
tell the user. Do not write or change scripts to work around the policy. See
skill lauterbach-t32mcp-execution-policy.
Address a script with skill_name and script_name: it is looked up as
<skill>/scripts/<script_name>.cmm inside the --skills directories of the
PowerView host, so the paths of the agent's machine and the host can differ.

{SKILLS_SOURCE}

The script header (the first JSON object in the script's leading comment block:
"interface", "strict", "arguments") is checked: the interface
version and script_args against the declared arguments: missing required
arguments (an argument is required unless it declares "optional": true),
unknown arguments,
enum and pattern. Argument problems reject the call under every policy, so do other header problems. The message
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
    - skill_name or script_name is invalid, or the script is not found in the --skills directories
    - Rejected by the execution policy: the script does not declare "strict": true in its header
    - An argument does not match its declaration in the script header
    - TRACE32 PowerView is not running or the API Port is not enabled
    - The script stopped at an error
    - Timeout: the script waits, e.g. in WAIT or a modal dialog (Windows)
    - PowerView does not respond: busy, hangs, or a modal dialog (Linux)