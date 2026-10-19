# Contributing to t32mcp

Thank you for your interest in this project. We welcome contributions via GitLab, both as issues and as merge requests.

## Reporting issues

Please use the [GitLab issue tracker](https://gitlab.com/lauterbach/t32mcp/-/issues) to report issues with the MCP server and the skills.

Include reproduction steps, your platform, and the relevant TRACE32 and MCP client versions.

## Submitting changes

Please submit changes as [GitLab merge requests](https://gitlab.com/lauterbach/t32mcp/-/merge_requests) against this repository. We accept:

- MCP server fixes: bug fixes and improvements to the server implementation in `t32mcp-rs/`.
- Skill additions: new or improved PRACTICE scripts in `skills/`, together with corresponding updates to the skill's `SKILL.md`. Every script must be documented there.

To create your own skills, use the bundled skill `skills/lauterbach-trace32-skill-creator`. It documents the script format, the script header, and how skills are written.

## A note on new MCP tools

We follow a progressive disclosure approach. The server intentionally exposes only a few generic tools (`trace32_run_script`, `trace32_run_command`, `trace32_get_status`, `trace32_manage_instances`).
New capabilities belong in PRACTICE scripts in the skills, not in new MCP tools.
We are therefore hesitant to accept merge requests that add new tools. 
If you believe a new tool is genuinely necessary, open an issue to discuss the rationale before submitting a merge request.
