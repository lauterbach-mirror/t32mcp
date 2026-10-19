Query the state of the TRACE32 target system.
Returns every PowerView instance this server talks to (--port,
trace32_manage_instances), in order: its Remote API 'port' and
'state' (connected or not reachable, with 'error').
Connected instances also report:
    - 'practice': the PRACTICE interpreter: 'state' (idle, running, paused
      e.g. in a dialog, or error), the call 'stack' of the active scripts
      (file, line, text, innermost first) and the open 'windows', which
      include named dialogs.
    - 'cores': every core of the instance (several for SMP) with its TRACE32
      core number 'core', 'selected' (the current core), CPU family, CPU,
      running flag and state line. Halted cores also report their registers;
      reading the registers of a running core is not possible.
Address an instance and core in the other tools with 'port' and 'core'.
The instances can belong to different targets.

The result is returned as JSON in the structured content of the tool call
(and as a text fallback containing the same JSON).

The connection to TRACE32 is established lazily on the first tool call and
re-established automatically if TRACE32 is (re)started, so a failed call can
be retried after launching PowerView.

Prerequisites:
    - TRACE32 PowerView must be running
    - API Port must be enabled in PowerView configuration