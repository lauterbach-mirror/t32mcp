---
name: trace32
description: Embedded Software Debugging with Lauterbach TRACE32
---

# Embedded Software Debugging with Lauterbach TRACE32

You have access to set of `*.cmm` scripts written in Lauterbach's scripting language PRACTICE that can be executed in the TRACE32 debugger.
To execute them, use Lauterbach's official MCP server `t32mcp`.

**Important**: The scripts are only visible to the MCP server. Do not try to read their contents. Expect that they will be empty.

## Build your strategy around hypothesis

Use the following strategy to help with debugging:

1. Formulate a hypothesis of what could be the problem
2. Test this hypothesis using the the provided scripts

Keep the answers relatively short - don't try too much, one hypothesis at a time.
If something is unclear, get back to the user.

## Control and Observe Dynamic State

Since debugging is a highly dynamic activity, you need to be able to control and observe the target system's state.

1. While you are reasoning, the target system should be in a halted state.
2. After reasoning, setup any traces you might need.
3. Start the execution for a specific amount of time.

### Examine the system from various perspectives

With the provided scripts, you can control and examine the system’s state from various perspectives.
Those perspectives help in identifying the suitable script. At the moment, the following perspectives are available:

* **Core:** Executes instructions
* **System on Chip**: Contains the cores as well as peripheral devices (e.g. interrupt controllers) which can be common sources of bugs as well, especially in embedded systems
* **Memory:** Stores information
* **Software:** Enables humans to write large sequences of instructions, includes common concepts like functions, typically in high-level languages like C
* **Operating System:** Ensures and simplifies dynamic runtime behavior of software through tasks, management routines etc.

There is no complete list of all possible perspectives.
For each use case, you must determine the most appropriate one to apply.

If you don't see anything applicable, get back to the user.

### Setup and Record Traces

Setting up traces is highly related with the hypothesis strategy.

1. Think first about which information will be interesting. Setup traces for those information with the appropriate scripts
2. Let the system run for a specific amount of time

After collecting the response, you can see the ordered trace information including timestamps.

## Script Reference

The `scripts/` directory contains a set of scripts implementing basic debugging functionality.
Some scripts can be called with arguments.

Always try to minimize the total amount of script calls, using the arguments as good as possible.

### Run Control

#### `get_state.cmm`

Get the target system's state.

#### `set_halted.cmm`

Halt the target system.

#### `set_running.cmm`

Continue the execution for `wait` milliseconds.
Parameter is optional. If not set, the target will keep running.
**IMPORTANT: Decimal values require a trailing dot, e.g. `12.`.**

```json
{
  "type": "object",
  "properties": {
    "wait": { "type": "string" }
  },
  "required": []
}
```

Example:

```json
{ }
{ "wait": "1000." }
```

### Core Perspective

#### `core_get_registers.cmm`

Access the core's current registers.


#### `core_set_register.cmm`

Set core register `register` to value `value`.

```json
{
  "type": "object",
  "properties": {
    "register": { "type": "string" },
    "value": {"type": "string"}
  },
  "required": ["register", "value"]
}
```

Example:

```json
{
    "register": "x1",
    "value": "0x10"
}
```

#### `core_get_instructions.cmm`

Read instructions around current program counter or a specified `address`.
**Note**: `address` can also be a symbol.

```json
{
  "type": "object",
  "properties": {
    "address": { "type": "string" }
  },
  "required": []
}
```

Example:

```json
{ }
{ "address": "0x1000" }
{ "address": "main" }
```

The line which the PC currently stands on is highlighted by underscores.

#### `core_step.cmm`

Execute `amount` of core instruction.
Parameter is optional. If not step, do just one step.
**IMPORTANT: Decimal values require a trailing dot, e.g. `12.`.**

```json
{
  "type": "object",
  "properties": {
    "count": { "type": "string" },
  },
  "required": []
}
```

Examples:

```json
{ }
{ "count": "10." }
```

### System on Chip (SoC) Perspective

Since the "execution environment" of the core can change, the SoC perspective is very important.

**Note:** The following scripts are general in their names but highly specific in their returned values.
If anything relevant is missing, get back to the user and tell them the required additional information.

#### `soc_get_state.cmm`

Get extensive information about the current state of the system on chip.
This may include:

- Control and status registers of the cores
- Information about peripherals

### Memory Perspective

**Note:** For the breakpoint and trace operations below, the `address` parameter can be either:
- An absolute memory address (e.g., `"0x1000"`)
- A symbol name for functions or variables (e.g., `"main"`, `"total"`, `"static_var"`)

Using symbols is often more convenient as you don't need to manually find addresses.

#### `mem_get_address.cmm`

Read the data at the specific memory address `address`.

```json
{
  "type": "object",
  "properties": {
    "address": { "type": "string" }
  },
  "required": ["address"]
}
```

Example:

```json
{ "address": "0x1000" }
```
#### `mem_dump_address.cmm`

Dump the data from memory starting at address `address`.

```json
{
  "type": "object",
  "properties": {
    "address": { "type": "string" }
  },
  "required": ["address"]
}
```

Example:

```json
{ "address": "0x1000" }
```

####  `mem_set_breakpoint.cmm`

Set a breakpoint at address `address`, `type` indicates whether it triggers when the address is accessed for read access, write access or execution.
`address` can also be a symbol (e.g. `main` or `static_var`).

```json
{
  "type": "object",
  "properties": {
    "address": { "type": "string" },
    "type": {"type": "string", "enum": ["R", "W", "X"] }
  },
  "required": ["address", "type"]
}
```

Example:

```json
{ "address": "0x100", "type": "R" }
{ "address": "static_var", "type": "W" }
{ "address": "main", "type": "X" }
```

#### `mem_trace_address.cmm`

Trace accesses to memory address `address`, `type` indicates whether it traces when the address is accessed for read access, write access or execution, respectively.
`address` can also be a symbol (e.g. `main` or `static_var`).

```json
{
  "type": "object",
  "properties": {
    "address": { "type": "string" },
    "type": {"type": "string", "enum": ["R", "W", "X"] }
  },
  "required": ["address", "type"]
}
```

Example:

```json
{ "address": "0x100", "type": "R" }
{ "address": "static_var", "type": "W" }
{ "address": "main", "type": "X" }
```

**Tip:** To trace both read and write access to a variable, call this script twice with the same address but different types (`R` and `W`).

#### `mem_clear_breakpoints.cmm`

Clears all active breakpoints and traces.

### Software Perspective

See `mem_*.cmm` scripts to control breakpoints and tracing for functions and variables.

#### `sw_get_locals.cmm`

Get all local variables.

#### `sw_get_stack_frame.cmm`

Get the current stack frame.

#### `sw_get_source.cmm`

Read source code around current program counter or a specified `address`.
**Note**: `address` can also be a symbol.

```json
{
  "type": "object",
  "properties": {
    "address": { "type": "string" }
  },
  "required": []
}
```

Example:

```json
{ }
{ "address": "0x1000" }
{ "address": "main" }
```

The line which the PC currently stands on is highlighted by underscores.

#### `sw_get_symbols.cmm`

Get all symbols matching the filter `wildcard` (e.g. `*main`).

```json
{
  "type": "object",
  "properties": {
    "wildcard": { "type": "string" }
  },
  "required": ["wildcard"]
}
```

Example:

```json
{ "wildcard": "*main"}
{ "wildcard": "static_var"}
```


#### `sw_step_over.cmm`

Step over the next `amount` of lines. If `amount` is not set, do one step.
**IMPORTANT: Decimal values require a trailing dot, e.g. `12.`.**

```json
{
  "type": "object",
  "properties": {
    "count": { "type": "string" },
  },
  "required": []
}
```

Examples:

```json
{ }
{ "count": "10." }
```

#### `sw_step_into.cmm`

Step into the next function

#### `sw_get_source_file.cmm`

Returns the contents of `filename`.

```json
{
  "type": "object",
  "properties": {
    "address": { "filename": "string" }
  },
  "required": ["filename"]
}
```

Example:

```json
{ "filename": "main.c" }
{ "filename": "lib.h" }
```

#### `sw_fgrep_source.cmm`

Searches all source files for occurrences of `pattern`.
**Important**: `pattern` is NOT a wildcard and NOT a regex.

```json
{
  "type": "object",
  "properties": {
    "pattern": { "type": "string" }
  },
  "required": ["pattern"]
}
```

Example:

```json
{ "pattern": "main"}
{ "pattern": "static_var"}
{ "pattern": "&&"}
```

#### `sw_set_breakpoint.cmm`

Set a breakpoint to `line` of `file`.

```json
{
  "type": "object",
  "properties": {
    "file": { "type": "string" },
    "line": {"type": "string" }
  },
  "required": ["file", "line"]
}
```

Example:

```json
{ "file": "main.c", "line": "13." }
{ "file": "lib.h", "line": "10." }
```

**IMPORTANT: Decimal values require a trailing dot, e.g. `12.`.**

## General instructions

### Refer to the user to reset the target

The above scripts do NOT provide the means to reset the target by yourself.
Instead, get back to the user.
It lies in their responsibility to bring the system back to a fresh state.
Ensure afterwards that you are aware of the state the system is in.
