---
name: lauterbach-trace32-skill-creator
description: Use this skill when the AI agent should gain new TRACE32 capabilities, e.g. create or extend an agent skill, add a PRACTICE script (*.cmm) to a skill so the agent can run it via t32mcp, make an existing collection of PRACTICE scripts available to the agent, or write evals for a TRACE32 skill. Not for scripts a human runs in PowerView (lauterbach-trace32-scripting).
license: "Apache-2.0, (C) 1989-2026 Lauterbach GmbH, see LICENSES/Apache-2.0.txt"
metadata:
  version: "0.1.0"
  date: "2026-10-02"
---

# Creating Skills for TRACE32

TRACE32 PowerView is fully scriptable in the PRACTICE language (`.cmm` files).
The debugger supports hundreds of commands, so skills follow progressive disclosure:
- A skill exposes a small set of documented PRACTICE scripts.
- The future agent loads only the skill it needs and calls the scripts via `t32mcp`.
- Common workflows live in durable skills, e.g. `lauterbach-trace32-debug-basic`.
- Specific workflows can be generated on demand, following the same conventions.

Your job: help the developer create or extend such a skill.

## `t32mcp`: Script Execution

`t32mcp` executes PRACTICE scripts in a running PowerView instance.

- Scripts live at `<skill>/scripts/<name>.cmm`, next to the skill's `SKILL.md`. The skill can be in any skills directory of the agent.
- Execution: `trace32_run_script` with `script_path` (absolute path), `script_args` (JSON object, all values strings), optional `port`, `core` and `timeout` (seconds, default 5).
- Scripts outside a skill directory also run, but the result starts with a warning.
- The user may restrict this with an execution policy. Under `strict`, the agent must not create or change scripts. See `lauterbach-t32mcp-execution-policy`.
- The JSON arguments arrive in the script as a single line of `key=value` pairs.
- Only one script runs at a time.
- Only what the script prints to the AREA window is returned to the agent. Print every result the agent needs.
- Do not switch the output AREA: no `AREA.Select`, `AREA.Create` (it selects the new AREA), `AREA.CLOSE` or `AREA.RESet`.
    - `t32mcp` reads the output from the AREA `A000`, which also receives the error and warning messages. Output printed after a switch is missing, closing ends the pipe.
    - Details: `lauterbach-t32mcp`, `references/area-and-pipe.md`.
- A script error ends the script. `t32mcp` returns the error message and the failing command. Still handle expected errors (`ON ERROR`) and print a clear message.

## `t32chat`: Knowledge Base

Use `t32chat` while authoring:
- `trace32_search_docs`: semantic search for the right PRACTICE command.
- `trace32_ask_expert`: questions answered from the manuals, with references.
- `trace32_check_syntax`: validate a script draft before executing it.

## Script Template

Scripts should roughly follow this template:

```cmm
; {
;   "interface": 1,
;   "strict": true,
;   "arguments": {
;     "name": { "type": "string" }
;   }
; }
;
; Example:
; { "name": "value" }

; ----- MCP tool script setup boilerplate ---------------
; prepare error handling and AREA window print output
ERROR.RESet
ON ERROR JUMPTO errorcleanup
PRinTer.Area

; get the current script path
LOCAL &script_file &abs_path
&script_file=OS.PresentPracticeFile()
&abs_path=OS.FILE.ABSPATH("&script_file")

; macros for input line and parsed input vars
PRIVATE &line &name
; read input line
ENTRY %LINE &line

; output header
PRINT ""
PRINT %WARNING "[MCP start] (&abs_path), args: &line"

; parse, validate, and transform input data
&name=STRing.SCANAndExtract("&line","name=","")
IF "&name"==""
(
  PRINT %ERROR "Error: parameter 'name' is required."
  PRINT %WARNING "[MCP end]"
  ENDDO
)

; ----- main script logic -------------------------------
; now use the input variables to perform all of the main logic and actions of the script
; should avoid jumps in here, as we expect to end up on the last line and then go to cleanup
; can use PRINT (with optional %ERROR and %WARNING modifiers) and the `WinPrint.` command prefix
; to add to the MCP tool output.

PRINT "Thank you for telling me your name, &name" "!"

; ----- cleanup and error handling boilerplate ----------
JUMPTO cleanup

errorcleanup:
(
  ; prevent further nested errors - could lead to infinite loop
  ON ERROR CONTinue
  ; error logs, this output is very helpful for understanding what went wrong where and fixing the script
  PRINT %ERROR "^^^ (error message on previous line): Encountered error during execution, caused by '" ERROR.CMDLINE() "'"
  ; sets the MCP status: t32mcp returns the output as a failed tool result (isError)
  PRINT %ERROR "[t32mcp: script failed]"
  ; output footer - duplicate, do not jump to cleanup, as an error there can lead to infinite loop
  PRINT %WARNING "[MCP end]"
  ; reset error
  ERROR.RESet
  ON ERROR inherit
  ENDDO
)

cleanup:
(
  ; optionally can print some final state here
  ; output footer
  PRINT %WARNING "[MCP end]"
  ON ERROR inherit
  ENDDO
)

```

Template rules:
- Comment header first: the interface version, `"strict": true` and the arguments, plus one example call. `t32mcp` checks every call against the header (see Schema).
- `"interface"`: the t32mcp script interface the script is written for. Required. Use the version of the other scripts. See `lauterbach-trace32-skills-version`.
- `PRIVATE &line <args>` plus `ENTRY %LINE &line` receives the arguments.
- `STRing.SCANAndExtract("&line","key=","default")` extracts one argument.
- All arguments arrive as strings. If of other type, need to be converted and validated.
- Arguments are data. Do not run them as commands or scripts (e.g. `&cmd` as a line, `DO &file`). If a script must do that, it must not declare `"strict": true` in its header.
- Decimal values require a trailing dot, e.g. `12.`, hex values `0x`. Strict radix mode rejects unqualified numbers (see `lauterbach-trace32-scripting`).
- Use `PRINT` for results, `PRINT %ERROR` for failures.
- Use `WinPrint` for window content:
  - `PRinTer.Area` first: routes the output to an AREA window, so it gets returned as MCP tool output.
  - `WinPrint.<window command>` then: prints the full window content, not only the visible part.
  - Examples: `WinPrint.List.Asm`, `WinPrint.TYPE`, `WinPrint.GREP`, `WinPrint.sYmbol.Browse`, `WinPrint.Register`.
- `ENDDO` must be used to exit the script, also usable as an early exit.
- Reset and restore error handling: `ERROR.RESet`, `ON ERROR CONTinue`, `ON ERROR inherit`.
- `PRINT %ERROR "[t32mcp: script failed]"` in `errorcleanup`: t32mcp then returns the output as a failed tool result. Exactly this text, on a line of its own.

## Workflow

1. Clarify what the script must do and which arguments it takes.
2. Check whether another skill script does something similar. Reuse as much as possible.
3. Check if a demo script already does something similar (see `lauterbach-trace32-scripting`). Reuse as much as possible adapt where necessary (see below).
4. (requires `t32chat` MCP server) Find the correct PRACTICE commands with `trace32_search_docs` or `trace32_ask_expert`.
5. Write the script into `<skill>/scripts/`, following the template.
6. (requires `t32chat` MCP server) Validate the draft with `trace32_check_syntax`.
    - Edit only the lines with errors. Keep edits minimal.
    - Re-run the check after every fix attempt.
    - The checker may report false positives for complex arguments, e.g. symbols with backslashes.
7. (requires `t32mcp` MCP server) Format the script with `beautify_script.cmm` from `lauterbach-trace32-scripting`.
8. (requires `t32mcp` MCP server)If a target system is connected, test the script via `trace32_run_script`.
9. Document the script in the skill's `SKILL.md`.
10. Add evals (see below) and run the linter.

## Schema

PRACTICE expands an argument as text into the command line. `Break.Set &address` with `A:main /CMD "DO x.cmm"` adds an option that runs a script.
The header declares what an argument may contain. `t32mcp` rejects the execution if a required argument is missing, an unknown argument is passed, an argument does not match its declaration, or the header has unknown keys. Still validate in the script.
- `"strict": true` declares the script compatible with the `strict` execution policy. Scripts without it are rejected under `strict`.
- Each argument is described by exactly one of:
    - `type`: one of the pre-defined types below.
    - `pattern`: a custom regular expression, anchored with `^` and `$`.
    - `enum`: a list of allowed values.
- `optional` (per argument, default `false`): set to `true` so a call may omit the argument. Every argument is required by default. A script without `arguments` takes no arguments.
- Pre-defined types, by use in the script:
    - Unquoted (`Break.Set &address`, `Register.Set &register &value`): `address` (`main`, `0x1000`, `A:main`), `symbol` (register or window name), `wildcard` (`main*`), `number` (`12.`, trailing dot for PRACTICE decimals). No space, `/`, `,`, `"`, `&`, `;` or parentheses.
    - Inside quotes (`Data.LOAD.Elf "&file"`): `string` (file path, free text). No `"`, no `&`.
- `uv run tools/lint_skills.py` checks the schemas and the example. Add a test case per argument that the schema rejects (`"rejected": true`).

## Adapting an Existing Script

Most existing scripts (demo scripts, the user's own collection) are written for a human at the GUI.
Run via `t32mcp`, they often return nothing or block. Adapt them before wrapping them in a skill:
- Output: `t32mcp` returns only the AREA window. Replace opened windows with `PRinTer.Area` plus `WinPrint.<command>`, and print results with `PRINT`.
- Remove AREA commands (`AREA.Select`, `AREA.Create`, `AREA.CLOSE`, `AREA.RESet`), see above.
- Dialogs: replace input dialogs with `ENTRY` arguments. `WAIT` for user input blocks until the `t32mcp` timeout.
- Never use modal dialogs (`DIALOG.OK`, `DIALOG.YESNO`, `DIALOG.File`). They have no name, so the agent cannot operate them. The script waits until a user clicks. On Linux, they also block the Remote API.
- Dialogs that must stay: `DIALOG.view` plus `STOP`, and give every dialog a name (`NAME "<name>"` in the definition, or `WinPOS ,,,,,, <name>` before it). The agent then operates it with `trace32_run_command`. Document the name and the element labels in `SKILL.md`. See `lauterbach-trace32-debug-ui`.
- GUI state: restore what the script changes, e.g. the current page (`WINPAGE.CURRENT()`, `WinPAGE.select`), also on the error path.
- Paths: use `~~~~/` (directory of the running script) instead of paths relative to the working directory.

## Where a New Skill Goes

- Durable, reusable workflows: a new skill next to the others in the skills root.
- Do not edit `lauterbach-*` skills. Library updates replace them and local changes are lost.
    - To extend: create an own skill next to them, e.g. with additional scripts.
    - To fix or improve an official skill: open an issue or a merge request in the Lauterbach skills repository. TODO(pfoth): public repository URL.
- Name user skills with an own prefix, e.g. `<company>-trace32-debug-<area>`.
- `metadata.version` in the frontmatter of `lauterbach-*` skills is the library version. Own skills keep their own version, or none.
- A skill directory name must equal the `name` in its frontmatter: lowercase, digits, hyphens.
- Tell the agent about project skills in the project's `CLAUDE.md` / `AGENTS.md` if the description alone does not route well.

## SKILL.md of a New Skill

Frontmatter: `name` and `description`. Nothing else.

The `description` is the only text the agent sees before loading the skill. Write it for routing:
- Imperative: "Use this skill when the user wants to ...".
- Describe the user's intent, not the implementation.
- Name implicit cases, e.g. "even if TRACE32 is not named".
- Name the boundary to neighbouring skills.
- At most 1024 characters, preferably below 400.

Body:
- Keep `SKILL.md` below 500 lines. Move long reference material to `references/<topic>.md` and say when to read it.
- Only write what the agent would get wrong without it. Prefer one default over a menu of options.
- End with a `## Gotchas` section: concrete corrections of mistakes observed in real runs.

Name scripts by perspective: `core_`, `soc_`, `mem_`, `sw_`, `os_`. Run control scripts have no prefix. See `lauterbach-trace32-debug-basic` for the full convention.

The script reference is what the agent sees instead of the script source. Per script:
- One-line purpose.
- One bullet per argument: name, required or optional, meaning, example value.
- One example call.

State global gotchas once, not per script.

Example entry:

```
#### `mem_set_breakpoint.cmm`
Set a breakpoint at `address` for access `type` (R, W or X).
- `address` (string, required): memory address or symbol, e.g. `main`, `static_var`
- `type` (string, required): "R", "W" or "X"

Example: { "address": "main", "type": "X" }
```

Script output:
- Print compact results. Large window dumps get truncated by the agent harness, so limit ranges and lines.
- Error messages say what was expected, e.g. `Error: parameter 'type' must be R, W or X.`

For a complete example, see `lauterbach-trace32-debug-basic`.

## Evals

Every skill in the library carries tests, following agentskills.io:
- `<skill>/evals/evals.json`: output-quality cases. Fields: `skill_name`, then `evals[]` with `id`, `prompt`, `expected_output`, optional `files` and `assertions`. Start with 2-3 realistic prompts. Add assertions after the first run.
- `evals/routing.json` in the repository root: routing queries. Add 3-5 entries per new skill with `query`, `expected_skill`, `not_skills`, optional `also_ok` (other acceptable skills) and `split` (`train` or `validation`). Prefer near-misses between neighbouring skills.
- Run `uv run tools/lint_skills.py` before committing.
- Run `uv run tools/run_routing_evals.py --skill <name>` to check the description routes correctly.
