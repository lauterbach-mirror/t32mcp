---
name: lauterbach-trace32-debug-basic
description: Use this skill when the user wants to inspect or control a running embedded target through TRACE32, e.g. halt/run/step, read registers or memory, set breakpoints or watchpoints, view source, locals or the call stack, or load an ELF. Use it even if TRACE32 is not named, as soon as a connected target should be debugged. To add new skills or scripts refer to lauterbach-trace32-skill-creator. Defines the script naming convention for all lauterbach-trace32-debug-* skills.
license: "Apache-2.0, (C) 1989-2026 Lauterbach GmbH, see LICENSES/Apache-2.0.txt"
metadata:
  version: "0.1.0"
  date: "2026-10-02"
---

# Basic Debugging with Lauterbach TRACE32

The scripts of this skill run in TRACE32 PowerView via Lauterbach's MCP server `t32mcp`.

## Calling the Scripts

1. Call `trace32_get_status` first. It returns every PowerView instance (`port`) with its cores (`core`), the CPU, the run state and, for halted cores, the registers.
2. Call `trace32_run_script` with:
    - `script_path`: absolute path `<this skill's directory>/scripts/<name>.cmm`, e.g. `.../lauterbach-trace32-debug-basic/scripts/get_state.cmm`
    - `script_args`: JSON object, all values strings, e.g. `{ "address": "main" }`
    - `port`, `core` (optional): instance and core from `trace32_get_status`. Needed for multicore/AMP setups. Without them, the selected core of the first reachable instance is used. Several instances: `lauterbach-trace32-debug-amp`. Several cores in one instance: `lauterbach-trace32-debug-smp`.
    - `timeout` (optional): seconds to wait, default 5. Raise it for long-running scripts.

## Tool Order

All `lauterbach-trace32-debug-*` skills follow this order:
1. `trace32_run_script` with a skill script. Check the script reference of the matching debug skill first.
2. No script fits: write one with `lauterbach-trace32-skill-creator`, if the task needs several commands or comes up again.
3. `trace32_run_command`: fallback for a single command that no script covers, e.g. `PRINT Register(PC)`.
    - Looking up the commands is recommended: `t32chat` (`trace32_search_docs`, `trace32_check_syntax`). Without `t32chat`, read the manuals (`lauterbach-trace32-docs`) or ask the user.
    - It returns only what is outputted to the AREA, e.g. via `PRINT` and `WinPrint.<command>`.
4. Do not drive the debugger by other means (Python, Remote API code, shell scripts, GUI automation) unless the user asks or there is no alternative.

`trace32_run_command` missing, or scripts rejected: the user set an execution policy. Follow `lauterbach-t32mcp-execution-policy`. Under `strict`, steps 2 and 3 are not allowed.

Setup, installation and troubleshooting of `t32mcp`: see skill `lauterbach-t32mcp`.

Note: some scripts may produce very large outputs (such as entire files, or large memory dumps).
When such large output is expected, you may write it to a file (if possible) and parse and filter it with other tools as necessary (to avoid flooding your context).

## Debugging Strategy

1. Formulate one hypothesis of what the problem could be.
2. Test it with the scripts, one hypothesis at a time.
3. If something is unclear, get back to the user.

Prefer the scripts over asking the user about the system state.
While reasoning, keep the target halted.
After reasoning, set up any traces you need, then let the target run for a specific amount of time.

## Naming Convention

Every `lauterbach-trace32-debug-*` skill names its scripts by perspective:
- no prefix: run control, e.g. `get_state.cmm`, `set_halted.cmm`
- `core_`: executes instructions (registers, disassembly, instruction steps)
- `soc_`: cores plus peripherals, e.g. interrupt controllers
- `mem_`: stores information (memory, data breakpoints)
- `sw_`: functions and variables in high-level code
- `os_`: tasks and runtime management of an operating system

Verbs: `get_` reads, `set_` changes, `list_` enumerates.
There is no complete list of perspectives. If nothing applies, get back to the user.

Specialized debug skills:
- `lauterbach-trace32-debug-flash`: program on-chip or off-chip flash
- `lauterbach-trace32-debug-trace`: record and analyze program flow trace
- `lauterbach-trace32-debug-coverage`: code coverage
- `lauterbach-trace32-debug-profiling`: function and task runtime
- `lauterbach-trace32-debug-rtos`: OS awareness, tasks

If a needed skill or script does not exist, write one with `lauterbach-trace32-skill-creator`.

## Script Reference

Minimize the total number of script calls. Use the arguments as well as possible.
Unless stated otherwise, `address` arguments accept an absolute address, a symbol name (e.g. `main`, `static_var`), or an access-class expression (e.g. `CSR:0x342` for a RISC-V CSR, `SPR:0x27E` for a PowerPC SPR).

### Run Control

#### `get_state.cmm`
Get the target system's state.

#### `set_halted.cmm`
Halt the target system. Prints the same state as `get_state.cmm`.

#### `set_running.cmm`
Let the target run for `wait` milliseconds.
- `wait` (string, optional): duration in milliseconds, e.g. `"1000."`. If not set, the target keeps running.
    Set the `t32mcp` `timeout` above `wait`.

Example: `{ "wait": "1000." }`

### Core

#### `core_get_registers.cmm`
Get the core's current registers.

#### `core_set_register.cmm`
Set core register `register` to `value`.
- `register` (string, required): register name, e.g. `"x1"`
- `value` (string, required): register value, e.g. `"0x10"`

Example: `{ "register": "x1", "value": "0x10" }`

#### `core_get_instructions.cmm`
Read instructions around the current program counter or at `address`. The current line is highlighted by underscores.
- `address` (string, optional): absolute address or symbol, e.g. `"0x1000"`, `"main"`

Example: `{ "address": "main" }`

#### `core_step.cmm`
Execute `count` core instructions. Prints the new program counter.
- `count` (string, optional): number of instructions, e.g. `"10."`. Default: one step.

Example: `{ "count": "10." }`

### System on Chip

#### `soc_get_state.cmm`
Get extensive state of the system on chip, e.g. control and status registers of the cores and peripheral information.
The script is general in name but highly specific in its returned values. If something relevant is missing, get back to the user.

### Memory

#### `mem_get_address.cmm`
Read the data at memory address `address`.
- `address` (string, required): absolute address or symbol

Example: `{ "address": "0x1000" }`

#### `mem_dump_address.cmm`
Dump the data starting at memory address `address`.
- `address` (string, required): absolute address or symbol

Example: `{ "address": "0x1000" }`

#### `mem_set_breakpoint.cmm`
Set a breakpoint at `address`, triggering on access `type`.
- `address` (string, required): absolute address or symbol, e.g. `"main"`, `"static_var"`
- `type` (string, required): `"R"`, `"W"` or `"X"` for read, write or execute access

Example: `{ "address": "main", "type": "X" }`

#### `mem_trace_address.cmm`
Trace accesses to memory address `address` with access `type`. Same arguments as `mem_set_breakpoint.cmm`.
Tip: to trace reads and writes on one address, call this script twice, with `"R"` and with `"W"`.

#### `mem_clear_breakpoints.cmm`
Clear all active breakpoints and traces.

### Software

For breakpoints and traces on functions or variables by symbol, see `mem_set_breakpoint.cmm` and `mem_trace_address.cmm` (use `"X"` for execution access).

#### `sw_get_locals.cmm`
Get all local variables.

#### `sw_get_stack_frame.cmm`
Get the current stack frame.

#### `sw_get_source.cmm`
Read source code around the current program counter or at `address`. The current line is highlighted by underscores.
- `address` (string, optional): absolute address or symbol, e.g. `"0x1000"`, `"main"`

Example: `{ "address": "main" }`

#### `sw_get_source_file.cmm`
Return the contents of `filename`.
- `filename` (string, required): absolute path, or a name found in the working directory or the source search path, e.g. `"/work/src/main.c"`. Get full paths with `sw_fgrep_source.cmm`.

Example: `{ "filename": "main.c" }`

#### `sw_get_symbols.cmm`
Get all symbols matching the wildcard `wildcard`.
- `wildcard` (string, required): wildcard pattern, e.g. `"*main"`

Example: `{ "wildcard": "*main" }`

#### `sw_step_over.cmm`
Step over the next `count` source lines. Prints the source around the new program counter.
- `count` (string, optional): number of lines, e.g. `"10."`. Default: one step.
- `imaskhll` (string, optional): set `SYStem.Option.IMASKHLL` before stepping — `"ON"` masks interrupts during HLL single-stepping, `"OFF"` allows interrupt preemption of steps. If omitted, the option is left unchanged. The setting persists after the script.

Example: `{ "count": "10." }`, `{ "count": "6.", "imaskhll": "OFF" }`

#### `sw_translate_sourcepath.cmm`
Translate source file paths from the build environment to the debug environment (command `sYmbol.SourcePATH.Translate`). Use when HLL source listing reports files not found.
- `from` (string, required): invalid path part as stored in the debug information, e.g. `"\work\src"`
- `to` (string, required): correct path part on the debug host, e.g. `"C:\project\src"`

Example: `{ "from": "\\work\\src", "to": "C:\\project\\src" }`

#### `sw_list_sourcepath.cmm`
List the source path translations, the loaded source files and the source files that are not found, with full paths. No arguments.

#### `sw_fgrep_source.cmm`
Search all source files for occurrences of `pattern`. Not a wildcard, not a regex.
- `pattern` (string, required): plain text, e.g. `"main"`, `"&&"`

Example: `{ "pattern": "static_var" }`

#### `sw_set_breakpoint.cmm`
Set a breakpoint at `line` of `file`.
- `file` (string, required): file name, e.g. `"main.c"`
- `line` (string, required): line number, always decimal, e.g. `"13."` or `"13"`

Example: `{ "file": "main.c", "line": "13." }`

#### `sw_step_into.cmm`
Step one source line, into called functions. Prints the source around the new program counter.

#### `sw_load_elf.cmm`
Load the ELF image `file` to the target. The target must be halted (see `set_halted.cmm`).
- `file` (string, required): path to the ELF file, e.g. `"app.elf"`

This does not program flash. For that use `lauterbach-trace32-debug-flash`.

Example: `{ "file": "app.elf" }`

## Reset the Target

The scripts do NOT provide the means to reset the target.
Get help from the user; they should always be able to bring the system back to a fresh state.
Ensure afterwards that you are aware of the state the system is in.

## Gotchas

- Decimal argument values require a trailing dot, e.g. `12.`. Without it, `12` means `0x12`, or fails with `number without hex or decimal qualifier not allowed in strict radix mode`. Hex values never get a dot.
- A script error fails the call with the error message and the failing command. Fix the argument, do not retry unchanged.
- A timed-out script keeps running. The next call fails with its call stack. Wait, or end it with `if_busy` `terminate`.
- Do not guess chip details such as register addresses or memory maps. Ask the user, or read them with `soc_get_state.cmm`.
