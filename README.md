# Lauterbach TRACE32 MCP Support (Preview)

Please note that this is an early development preview. The implementation is still evolving and may change in future releases.

**Questions & Feedback**:
We would love to get your feedback and improvement suggestions for our MCP implementation. For questions, feedback and help, please use the GitLab issues feature.

## Table of Contents
1. [Files overview](#files-overview)
2. [MCP server](#mcp-server)
3. [Agent skills](#agent-skills)
4. [AI agent configuration](#ai-agent-configuration)
5. [Intended agent behavior](#intended-agent-behavior)
6. [Concept and AI roadmap](#concept-and-ai-roadmap)

## Files overview

```
t32mcp/                                # this repository
├── README.md                          # this file
├── t32mcp-rs/                         # the MCP server source code
├── CLAUDE.md                          # debugging instructions example
└── .claude/
    └── skills/
        └── skill-trace32/             # Accompanying Agent Skill
            ├── SKILL.md               # MCP usage and script documentation
            └── scripts/
                ├── <script-name>.cmm  # the "actual tools" implemented as PRACTICE scripts
                └── ...
```

## MCP server

You can either download and use one of the prebuilt executables (if compatible) or build it yourself from source.
**Note:** the path to the executable needs to be accessible by the MCP client (your AI agent, e.g. Claude Code), and the executable needs to be able to communicate with TRACE32 via the remote API (TCP by default).

For simplicity the rest of this README assumes the executable sits at `/<your-path>/t32mcp/t32mcp` / `C:\<your-path>\t32mcp\t32mcp.exe`.

### Download a prebuilt release (recommended)

Prebuilt executables for x86-64 Linux and Windows are attached to each release.
Grab them from the [releases page](https://gitlab.com/lauterbach/t32mcp/-/releases):
- `t32mcp-linux-x86_64`
- `t32mcp-windows-x86_64.exe`
- `skill-trace32.zip` for the accompanying TRACE32 agent skill (`skill-trace32/` with `SKILL.md` and the PRACTICE scripts)

Download the executable for your platform and the skill zip, then unzip `skill-trace32.zip` to wherever you keep your skills (see [Skills](#skills)). On Linux make the executable runnable with `chmod +x t32mcp-linux-x86_64`. You may rename and move the executable to your preferred location.

### Build it yourself

The Rust source code for the MCP server is provided in the `t32mcp-rs` directory. Use `cargo build --release` in that directory to get the portable executable `t32mcp` (Linux) / `t32mcp.exe` (Windows) (in `t32mcp/t32mcp-rs/target/release/`). You may move it to your preferred location.

### Usage

The MCP server communicates with the MCP client via the `stdio` protocol.

The executable should be called with the command-line parameter `--skills <path-to-skills>`. This points the MCP server to the root directory from which it loads the PRACTICE scripts `<path-to-skills>/skill-trace32/scripts/*.cmm`.

There is one more, optional parameter: `--port`. It defaults to 20000 (the default port for the TRACE32 remote API). If you use a different port, provide it here.

### Tools

The MCP server exposes three tools:

1. `execute_practice_skill`: to execute a script
2. `abort_practice_skill`: to cancel the current script
3. `collect_practice_skill_response`: to get / poll the results of a running script


## Agent skills

Our MCP server only provides the basic interface to TRACE32.
We then rely on the "skills" functionality to explain how to use these MCP tools.
Currently, all instructions and documentation are in the `SKILL.md` file.

### Scripts

The skill's `scripts` directory contains a collection of PRACTICE (`*.cmm`) scripts that provide the actual tool capabilities for the AI agent.

Scripts can take input arguments and can return information to the AI agent by printing to the `AREA` window.
Take the very simple `sw_get_stack_frame.cmm` as an example:
```cmm
PRinTer.Area
WinPrint.Frame

ENDDO
```

**Note:** Every script and its usage should be documented in the `SKILL.md` file!

## AI agent configuration

### MCP

To add our MCP server, follow the instructions for your AI agent. Usually, this includes specifying the protocol, which should be set to `stdio` (sometimes also `local`).
Then set the command / path to the executable `/<your-path>/t32mcp/t32mcp` / `C:\<your-path>\t32mcp\t32mcp.exe`.

**Important:** also add the command argument `--skills <path-to-your-skills>`. What to set for `<path-to-your-skills>` will depend on how you configured your skills (next section).

### Skills

Most AI agents support skills from multiple standard directories (both global and local / project-based), e.g. in `.claude/skills/`.

Place our `skill-trace32` skill into one of those supported skill directories (referenced here as `<path-to-your-skills>`). Either unzip the released `skill-trace32.zip` there, or copy it from a checkout at `<your-path>/t32mcp/.claude/skills/skill-trace32`. Then use that directory `<path-to-your-skills>` for the `--skills` argument when adding the MCP server!

**Important:** If you customize the skill and scripts, e.g. by adding new scripts, **you must point the MCP to the correct skill directory with the `--skills` argument**. If you instead point the MCP to the unmodified original git checkout, the AI agent will think it has access to the new functionality, however, the MCP server will not be able to find the corresponding PRACTICE scripts!

**Note:** If you have copied `skill-trace32` to both local and global skill directories (and potentially even edited them so they differ), make sure you understand your AI agent's priority order that determines which skill is actually loaded.

#### Customization

In your local project you can freely edit `skill-trace32/SKILL.md` and the corresponding PRACTICE scripts to customize the AI agent's TRACE32 interface and instructions, without affecting other projects or the original configuration. You could also add new skills such as `skill-trace32-arm-etm-trace`. If you already have a library of useful PRACTICE scripts, you can also make these available to the AI agent through skills.

### Claude Code example

This registers the MCP server with Claude Code. It assumes you have already placed `skill-trace32` into a skills directory (see [Skills](#skills)). Substitute your own two paths:
- the MCP server executable
- the skills directory that contains `skill-trace32`

The command is the same on every platform; only the path syntax differs.

Linux example:
```bash
claude mcp add-json t32mcp '{"type":"stdio","command":"/home/you/t32mcp/t32mcp","args":["--skills", "/home/you/my-project/.claude/skills"]}'
```

Windows (PowerShell) example:
```powershell
claude mcp add-json t32mcp '{"type":"stdio","command":"C:\\Users\\you\\t32mcp\\t32mcp.exe","args":["--skills", "C:\\Users\\you\\my-project\\.claude\\skills"]}'
```

(Optional) Use our "debug instructions" by copying the provided `CLAUDE.md` into your project root / into your own `CLAUDE.md`. Note that we have not evaluated how well these work, feel free to change.

Note: to remove the MCP server from Claude Code again, use `claude mcp remove t32mcp`.


## Intended agent behavior

If you ask your agent to use TRACE32 to set a register value on your target, it should automatically read the `skill-trace32` skill. In the `SKILL.md` file it will find:

````md
You have access to a set of `*.cmm` scripts written in Lauterbach's scripting language PRACTICE that can be executed in the TRACE32 debugger.
To execute them, use Lauterbach's official MCP server `t32mcp`.

...

## Script Reference

...

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
````

It should then check the tools provided by the MCP server and should decide to use the `execute_practice_skill` tool to execute the `core_set_register.cmm` script with the corresponding parameters.

## Concept and AI roadmap

Ideally, an AI agent would be able to use TRACE32 in the same way as a human (expert).
Conveniently for LLMs, TRACE32 can be fully controlled via text commands (PRACTICE / `cmm` scripts).
Unfortunately, there are a lot of PRACTICE commands and general-purpose LLMs do not seem to know much about them. This leads to very high hallucination rates for TRACE32-related information (especially when not using web-search tools).

A common way to mitigate this is to directly provide the TRACE32 manuals to the AI agent.
Indeed, we have invested some effort in providing information and additional tools to make the AI a more useful TRACE32 assistant. The TRACE32 AI Assistant is available as a chatbot on the website [https://assistant.ai.lauterbach.com](https://assistant.ai.lauterbach.com) (limited preview, please sign up with your company email address to increase the likelihood of getting access).

We also plan to make the chatbot functionality available as a remote MCP server, such that your AI agent can benefit from those same informed TRACE32 answers and resources.
Once that is available, we could also try allowing the AI agent to interface with TRACE32 directly via PRACTICE (as a human expert would), truly "unlocking" all TRACE32 features for the AI to use.

But until then we need some sort of "wrapper interface" to TRACE32 that the AI can understand. For now we opted for the "MCP + Agent skill + PRACTICE scripts" approach described above, since it also allows for user customization and extensions while allowing the AI to access virtually any TRACE32 functionality (once provided as a script).
In the meantime **you can have our TRACE32 AI Assistant help you write your own skills and extension PRACTICE scripts!**


# License

This project is licensed under the Apache License, Version 2.0. See the
[LICENSE](LICENSE) file for details.
