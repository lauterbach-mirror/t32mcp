// SPDX-FileCopyrightText: 2026 Lauterbach GmbH
// SPDX-License-Identifier: Apache-2.0

//! Documentation templates for the agent (docs/), with their placeholders
//! filled in: the server instructions and the tool descriptions.

use crate::execution_policy;

/// Default for --skills-source: t32chat, which serves the skills.
pub const DEFAULT_SKILLS_SOURCE: &str = "https://assistant.ai.lauterbach.com";

/// Natural-language guidance for LLMs where to get the skills from
pub fn skills_source(policy: execution_policy::ExecutionPolicy, source: &str) -> String {
    // The install commands on that page contain a short-lived download ticket of the
    // logged-in user, so the agent cannot build them itself.
    let page = include_str!("docs/skills_page.md").replace("{SOURCE}", source);
    match policy {
        execution_policy::ExecutionPolicy::Strict => format!(
            "The scripts come from the lauterbach-* agent skills. If they are not\ninstalled or outdated, ask the user to install them: {page}\nThey must be local files on this host."
        ),
        _ => include_str!("docs/skills_source_default.md")
            .replace("{SOURCE}", source)
            .replace("{PAGE}", &page),
    }
}

/// Natural-language guidance for LLMs on how to use the MCP server
pub fn server_instructions(policy: execution_policy::ExecutionPolicy, source: &str) -> String {
    match policy {
        execution_policy::ExecutionPolicy::Unrestricted => {
            include_str!("docs/server_instructions_unrestricted.md")
                .replace("{VERSION}", env!("CARGO_PKG_VERSION"))
                .replace(
                    "{INTERFACE}",
                    &execution_policy::SCRIPT_INTERFACE_VERSION.to_string(),
                )
                .replace("{SKILLS_SOURCE}", &skills_source(policy, source))
        }
        execution_policy::ExecutionPolicy::Scripts => {
            include_str!("docs/server_instructions_scripts.md")
                .replace("{VERSION}", env!("CARGO_PKG_VERSION"))
                .replace(
                    "{INTERFACE}",
                    &execution_policy::SCRIPT_INTERFACE_VERSION.to_string(),
                )
                .replace("{SKILLS_SOURCE}", &skills_source(policy, source))
        }
        execution_policy::ExecutionPolicy::Strict => {
            include_str!("docs/server_instructions_strict.md")
                .replace("{VERSION}", env!("CARGO_PKG_VERSION"))
                .replace(
                    "{INTERFACE}",
                    &execution_policy::SCRIPT_INTERFACE_VERSION.to_string(),
                )
                .replace("{SKILLS_SOURCE}", &skills_source(policy, source))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instructions_contain_skills_source() {
        // Where to get the skills, in every policy.
        for policy in [
            execution_policy::ExecutionPolicy::Unrestricted,
            execution_policy::ExecutionPolicy::Scripts,
            execution_policy::ExecutionPolicy::Strict,
        ] {
            assert!(
                server_instructions(policy, "http://localhost:8000")
                    .contains("http://localhost:8000/agents/skills")
            );
        }
        assert!(
            !server_instructions(
                execution_policy::ExecutionPolicy::Strict,
                DEFAULT_SKILLS_SOURCE
            )
            .contains("trace32_run_command:")
        );
    }
}
