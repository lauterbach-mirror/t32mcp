// SPDX-FileCopyrightText: 2026 Lauterbach GmbH
// SPDX-License-Identifier: Apache-2.0

//! The execution policy influences the set of tools provided by t32mcp
//! and how they are handled when invoked.

use regex::Regex;
use serde_json::{Map, Value};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ArgumentType {
    Address,
    Symbol,
    Wildcard,
    Str,
    Number,
}

impl ArgumentType {
    const ALL: [Self; 5] = [
        Self::Address,
        Self::Symbol,
        Self::Wildcard,
        Self::Str,
        Self::Number,
    ];

    /// The "type" name as used in the script header.
    fn name(self) -> &'static str {
        match self {
            Self::Address => "address",
            Self::Symbol => "symbol",
            Self::Wildcard => "wildcard",
            Self::Str => "string",
            Self::Number => "number",
        }
    }

    /// The regex that values of this type must match.
    fn pattern(self) -> &'static str {
        match self {
            Self::Address => "^[A-Za-z0-9_:.+\\-\\[\\]]+$",
            Self::Symbol => "^[A-Za-z0-9_]+$",
            Self::Wildcard => "^[A-Za-z0-9_:.*?\\[\\]]+$",
            Self::Str => "^[^\"&\\x00-\\x1f]+$",
            Self::Number => "^[0-9]+\\.?$",
        }
    }

    /// The pattern is a constant, so this cannot fail.
    fn regex(self) -> Regex {
        Regex::new(self.pattern()).expect("predefined pattern")
    }

    fn parse(t: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|ty| ty.name() == t)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExecutionPolicy {
    /// trace32_run_command and any script (non-skill scripts get a warning).
    Unrestricted,
    /// Any script, no trace32_run_command.
    Scripts,
    /// Only skill scripts that declare "strict": true, no trace32_run_command,
    /// no trace32_manage_instances.
    Strict,
}

impl ExecutionPolicy {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "unrestricted" => Some(Self::Unrestricted),
            "scripts" => Some(Self::Scripts),
            "strict" => Some(Self::Strict),
            _ => None,
        }
    }

    pub fn run_command_enabled(self) -> bool {
        self == Self::Unrestricted
    }
}

/// The script interface version of this t32mcp
pub const SCRIPT_INTERFACE_VERSION: u32 = 1;

/// A header problem that does not reject the script under the current
/// execution policy (script can run anyway)
fn header_warning(problem: &str) -> String {
    format!("Warning: {problem} The script runs anyway (the execution policy allows any script).")
}

#[derive(Debug, Default)]
pub struct ScriptHeader {
    /// The interface version the script was written for, if declared.
    pub interface: Option<u32>,
    /// `"strict": true`: the script opts in to the strict execution policy.
    pub strict: bool,
    /// The arguments the script accepts. Each argument is required unless
    /// it declares `"optional": true`.
    arguments: Vec<Property>,
    /// Header problems that do not reject the script under the policy it
    /// was parsed with. Empty under `strict`.
    pub warnings: Vec<String>,
}

#[derive(Debug)]
struct Property {
    name: String,
    /// `"optional": true`: a call may omit the argument.
    optional: bool,
    enum_values: Option<Vec<String>>,
    pattern: Option<Regex>,
}

impl ScriptHeader {
    /// Parses the leading comment block of a script and enforces policy:
    /// Under `strict`, the first header problem rejects the script
    /// Under the other policies the script runs anyway, its header problems are
    /// collected in `warnings`.
    pub fn parse(source: &str, policy: ExecutionPolicy) -> Result<Self, String> {
        let strict = policy == ExecutionPolicy::Strict;
        let mut header = match Self::parse_json(source) {
            Ok(header) => header,
            Err(e) if strict => return Err(e),
            Err(e) => {
                return Ok(Self {
                    warnings: vec![header_warning(&e)],
                    ..Self::default()
                });
            }
        };
        // With another interface version, the header may mean something else.
        // Only under strict must it be declared; without it the script may
        // as well be a demo script.
        let version = env!("CARGO_PKG_VERSION");
        let problem = match header.interface {
            None if strict => Some(format!(
                "Version mismatch: this skill script does not declare its t32mcp interface version (\"interface\": <n> in the script header). This t32mcp ({version}) supports interface v.{SCRIPT_INTERFACE_VERSION}. Update the skills. See skill lauterbach-trace32-skills-version."
            )),
            None => None,
            Some(i) if i > SCRIPT_INTERFACE_VERSION => Some(format!(
                "Version mismatch: the script is written for t32mcp interface {i}, this t32mcp ({version}) supports interface {SCRIPT_INTERFACE_VERSION}. Upgrade t32mcp. See skill lauterbach-trace32-skills-version."
            )),
            Some(i) if i != SCRIPT_INTERFACE_VERSION => Some(format!(
                "Version mismatch: the script defines an invalid t32mcp interface {i} (expected: {SCRIPT_INTERFACE_VERSION}). See skill lauterbach-trace32-skills-version."
            )),
            Some(_) => None,
        };
        if let Some(e) = problem {
            if strict {
                return Err(e);
            }
            header.warnings.push(header_warning(&e));
        }
        if strict && !header.strict {
            return Err("Rejected by execution policy strict (set by the user): the script does not declare \"strict\": true in its header. See skill lauterbach-t32mcp-execution-policy.".to_string());
        }
        Ok(header)
    }

    /// Validates the arguments of a call against the header. Tool errors
    /// under every policy: a mismatching argument could otherwise inject
    /// options into the script's command lines.
    pub fn validate_args(&self, args: Option<&Value>) -> Result<(), String> {
        let empty = Map::new();
        let args = match args {
            None | Some(Value::Null) => &empty,
            Some(Value::Object(map)) => map,
            Some(_) => return Err("script_args must be an object".to_string()),
        };
        if self.arguments.is_empty() {
            if args.is_empty() {
                Ok(())
            } else {
                Err("This script takes no arguments: its header declares no arguments".to_string())
            }
        } else {
            self.validate(args)
        }
    }

    /// The header is the first JSON object in the leading comment block.
    /// What follows it (e.g. an "; Example:" comment) is not parsed.
    fn parse_json(source: &str) -> Result<Self, String> {
        let comments: Vec<&str> = source
            .lines()
            .map(str::trim)
            .take_while(|l| l.is_empty() || l.starts_with(';') || l.starts_with("//"))
            .filter_map(|l| l.strip_prefix(';').or_else(|| l.strip_prefix("//")))
            .map(str::trim)
            .collect();

        // The header starts at the first line that opens a JSON object and
        // ends at the first line at which the JSON is complete.
        let mut json: Option<String> = None;
        for line in &comments {
            if json.is_none() && !line.starts_with('{') {
                continue;
            }
            let buffer = json.get_or_insert_with(String::new);
            buffer.push_str(line);
            buffer.push('\n');
            if let Ok(value) = serde_json::from_str::<Value>(buffer) {
                return Self::from_value(&value);
            }
        }
        match json {
            None => Ok(Self::default()),
            Some(_) => Err(
                "Invalid script header: the JSON object in the leading comment block is \
incomplete or no valid JSON"
                    .into(),
            ),
        }
    }

    fn from_value(value: &Value) -> Result<Self, String> {
        let object = value
            .as_object()
            .ok_or("Invalid script header: the header is no JSON object")?;
        // A keyword that is not checked would only look like a restriction.
        let allowed = ["interface", "strict", "arguments"];
        if let Some(k) = object.keys().find(|k| !allowed.contains(&k.as_str())) {
            return Err(format!(
                "Invalid script header: unsupported keyword \"{k}\" in the header. \
Supported: {}",
                allowed.join(", ")
            ));
        }
        let interface =
            match object.get("interface") {
                None => None,
                Some(v) => Some(v.as_u64().and_then(|i| u32::try_from(i).ok()).ok_or_else(
                    || "Invalid script header: \"interface\" must be a number".to_string(),
                )?),
            };
        let strict = match object.get("strict") {
            None => false,
            Some(Value::Bool(b)) => *b,
            Some(_) => return Err("Invalid script header: \"strict\" must be true or false".into()),
        };
        let mut arguments = Vec::new();
        if let Some(a) = object.get("arguments") {
            let props = a
                .as_object()
                .ok_or("Invalid script header: \"arguments\" is no JSON object")?;
            for (name, prop) in props {
                arguments.push(Property::parse(name, prop)?);
            }
        }
        Ok(Self {
            interface,
            strict,
            arguments,
            warnings: Vec::new(),
        })
    }

    /// Validates the arguments: missing required arguments (an argument is
    /// required unless it declares `"optional": true`), unknown
    /// arguments, non-string values, and values that match neither the enum
    /// nor the pattern of their argument.
    fn validate(&self, args: &Map<String, Value>) -> Result<(), String> {
        for prop in &self.arguments {
            if !prop.optional && !args.contains_key(&prop.name) {
                return Err(format!("Missing required argument '{}'", prop.name));
            }
        }
        for (name, value) in args {
            let Some(prop) = self.arguments.iter().find(|p| &p.name == name) else {
                return Err(format!(
                    "Unknown argument '{name}'. Arguments: {}",
                    self.arguments
                        .iter()
                        .map(|p| p.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            };
            let Some(value) = value.as_str() else {
                return Err(format!("Argument '{name}' must be a string"));
            };
            if let Some(values) = &prop.enum_values
                && !values.iter().any(|v| v == value)
            {
                return Err(format!(
                    "Argument '{name}' must be one of {values:?}, got {value:?}"
                ));
            }
            if let Some(pattern) = &prop.pattern
                && !pattern.is_match(value)
            {
                return Err(format!(
                    "Argument '{name}' does not match the pattern {}, got {value:?}",
                    pattern.as_str()
                ));
            }
        }
        Ok(())
    }
}

impl Property {
    /// An argument is described by exactly one of `"type"` (a predefined
    /// type), `"pattern"` (a custom anchored regex) or `"enum"` (its allowed
    /// values). It is required unless it declares `"optional": true`.
    fn parse(name: &str, value: &Value) -> Result<Self, String> {
        let object = value
            .as_object()
            .ok_or_else(|| format!("Invalid script header: argument '{name}' is no object"))?;
        // A keyword that is not checked would only look like a restriction.
        let allowed = ["type", "pattern", "enum", "optional"];
        if let Some(k) = object.keys().find(|k| !allowed.contains(&k.as_str())) {
            return Err(format!(
                "Invalid script header: unsupported keyword \"{k}\" in argument '{name}'. \
Supported: {}",
                allowed.join(", ")
            ));
        }
        let predefined = ArgumentType::ALL
            .iter()
            .map(|t| t.name())
            .collect::<Vec<_>>()
            .join(", ");
        let spec = [
            object.get("type"),
            object.get("pattern"),
            object.get("enum"),
        ]
        .iter()
        .filter(|v| v.is_some())
        .count();
        if spec != 1 {
            return Err(format!(
                "Invalid script header: argument '{name}' needs exactly one of \"type\" \
({predefined}), an anchored \"pattern\" or an \"enum\""
            ));
        }
        let optional = match object.get("optional") {
            None => false,
            Some(Value::Bool(b)) => *b,
            Some(_) => {
                return Err(format!(
                    "Invalid script header: \"optional\" of '{name}' must be true or false"
                ));
            }
        };
        if let Some(t) = object.get("type") {
            let t = t.as_str().ok_or_else(|| {
                format!(
                    "Invalid script header: \"type\" of argument '{name}' must be one of: {predefined}"
                )
            })?;
            let ty = ArgumentType::parse(t).ok_or_else(|| {
                format!(
                    "Invalid script header: unknown type \"{t}\" of argument '{name}', \
expected one of: {predefined}"
                )
            })?;
            return Ok(Property {
                name: name.to_string(),
                optional,
                enum_values: None,
                pattern: Some(ty.regex()),
            });
        }
        if let Some(p) = object.get("pattern") {
            let p = p.as_str().ok_or_else(|| {
                format!("Invalid script header: \"pattern\" of '{name}' is no string")
            })?;
            // Unanchored, a pattern only has to match a part of the value,
            // e.g. "[0-9]+" also matches "1 /CMD ...".
            if !p.starts_with('^') || !p.ends_with('$') {
                return Err(format!(
                    "Invalid script header: \"pattern\" of '{name}' must start with ^ and end with $"
                ));
            }
            return Ok(Property {
                name: name.to_string(),
                optional,
                enum_values: None,
                pattern: Some(
                    Regex::new(p).map_err(|e| {
                        format!("Invalid script header: \"pattern\" of '{name}': {e}")
                    })?,
                ),
            });
        }
        let values = object
            .get("enum")
            .and_then(|e| e.as_array())
            .ok_or_else(|| {
                format!("Invalid script header: \"enum\" of '{name}' is no list of strings")
            })?;
        let enum_values = values
            .iter()
            .map(|v| v.as_str().map(String::from))
            .collect::<Option<Vec<String>>>()
            .ok_or_else(|| {
                format!("Invalid script header: \"enum\" of '{name}' is no list of strings")
            })?;
        Ok(Property {
            name: name.to_string(),
            optional,
            enum_values: Some(enum_values),
            pattern: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const UNRESTRICTED: ExecutionPolicy = ExecutionPolicy::Unrestricted;
    const STRICT: ExecutionPolicy = ExecutionPolicy::Strict;

    const SCRIPT: &str = r#"; Set a breakpoint.
;
; {
;   "interface": 1,
;   "strict": true,
;   "arguments": {
;     "address": { "type": "address" },
;     "access": { "enum": ["R", "W", "X"], "optional": true },
;     "note": { "type": "string", "optional": true }
;   }
; }
;
; Example:
; { "address": "main", "access": "X" }
"#;

    fn header(script: &str) -> ScriptHeader {
        ScriptHeader::parse(script, UNRESTRICTED).unwrap()
    }

    #[test]
    fn predefined_regexes() {
        // Each compiled regex reports the pattern it was built from, the
        // mismatch error message relies on that (Regex::as_str).
        for ty in ArgumentType::ALL {
            assert_eq!(ty.regex().as_str(), ty.pattern());
        }
        for (ty, value) in [
            (ArgumentType::Address, "A:main"),
            (ArgumentType::Address, "0x1000"),
            (ArgumentType::Symbol, "core_0"),
            (ArgumentType::Wildcard, "main*"),
            (ArgumentType::Str, "C:/path with space/file.cmm"),
            (ArgumentType::Number, "12."),
        ] {
            assert!(ty.regex().is_match(value), "{}: {value}", ty.name());
        }
    }

    #[test]
    fn parse_header() {
        let header = ScriptHeader::parse(SCRIPT, STRICT).unwrap();
        assert_eq!(header.interface, Some(1));
        assert!(header.strict);
        assert!(header.warnings.is_empty());
        assert_eq!(header.arguments.len(), 3);
        let required: Vec<&str> = header
            .arguments
            .iter()
            .filter(|p| !p.optional)
            .map(|p| p.name.as_str())
            .collect();
        assert_eq!(required, vec!["address"]);
    }

    #[test]
    fn parse_header_without_json() {
        // No JSON object in the comment block: a demo script, no header.
        let header = ScriptHeader::parse("; Just a demo script.\n\nENDDO\n", UNRESTRICTED).unwrap();
        assert_eq!(header.interface, None);
        assert!(!header.strict);
        assert!(header.arguments.is_empty());
        assert!(header.warnings.is_empty());
        // Only the leading comment block counts.
        let header = ScriptHeader::parse("ENDDO\n; { \"interface\": 1 }\n", UNRESTRICTED).unwrap();
        assert_eq!(header.interface, None);
    }

    #[test]
    fn parse_header_incomplete() {
        let err = ScriptHeader::parse("; {\n;   \"interface\": 1\n\nENDDO\n", STRICT).unwrap_err();
        assert!(err.contains("incomplete"), "{err}");
        // A single-line object works.
        let header = ScriptHeader::parse("; { \"interface\": 1 }\n", UNRESTRICTED).unwrap();
        assert_eq!(header.interface, Some(1));
    }

    #[test]
    fn strict_opt_in() {
        // "strict": true is an opt-in: without it, the script does not run
        // under the strict policy.
        let err = ScriptHeader::parse("; { \"interface\": 1 }", STRICT).unwrap_err();
        assert!(err.contains("Rejected by execution policy strict"), "{err}");
        assert!(err.contains("\"strict\": true"), "{err}");
        // Under the other policies, the script runs anyway.
        for (script, strict) in [
            ("; { \"interface\": 1 }", false),
            ("; { \"strict\": false }", false),
            ("; { \"strict\": true }", true),
        ] {
            let header = ScriptHeader::parse(script, UNRESTRICTED).unwrap();
            assert_eq!(header.strict, strict, "{script}");
        }
    }

    #[test]
    fn predefined_types() {
        for (script, prop, value, ok) in [
            (
                "; { \"arguments\": { \"a\": { \"type\": \"address\" } } }",
                "a",
                "A:main",
                true,
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"address\" } } }",
                "a",
                "0x1000",
                true,
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"address\" } } }",
                "a",
                "main*",
                false,
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"symbol\" } } }",
                "a",
                "core_0",
                true,
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"symbol\" } } }",
                "a",
                "main:0",
                false,
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"wildcard\" } } }",
                "a",
                "main*",
                true,
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"wildcard\" } } }",
                "a",
                "main?",
                true,
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"wildcard\" } } }",
                "a",
                "C:/x",
                false,
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"string\" } } }",
                "a",
                "C:/path with space/file.cmm",
                true,
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"string\" } } }",
                "a",
                "a\"b",
                false,
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"string\" } } }",
                "a",
                "a&b",
                false,
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"number\" } } }",
                "a",
                "12.",
                true,
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"number\" } } }",
                "a",
                "12",
                true,
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"number\" } } }",
                "a",
                "0x12",
                false,
            ),
        ] {
            let result = header(script).validate_args(Some(&json!({ prop: value })));
            assert_eq!(result.is_ok(), ok, "{script} {prop}={value}: {result:?}");
        }
    }

    #[test]
    fn validate() {
        let header = header(SCRIPT);
        assert!(
            header
                .validate_args(Some(&json!({"address": "main", "access": "X"})))
                .is_ok()
        );
        // Option injection, as in Break.Set &address.
        let err = header
            .validate_args(Some(&json!({"address": "A:main /CMD \"DO x.cmm\""})))
            .unwrap_err();
        assert!(
            err.contains("does not match the pattern ^[A-Za-z0-9_:.+\\-\\[\\]]+$"),
            "{err}"
        );
        assert!(
            header
                .validate_args(Some(&json!({"address": "main", "access": "Z"})))
                .is_err()
        );
        // Missing required and unknown arguments are errors under every policy.
        assert_eq!(
            header
                .validate_args(Some(&json!({"access": "X"})))
                .unwrap_err(),
            "Missing required argument 'address'"
        );
        // SCANAndExtract("&line","address=",...) could find the key in another value.
        let err = header
            .validate_args(Some(&json!({"address": "main", "x": "address=A:0 /CMD"})))
            .unwrap_err();
        assert!(err.contains("Unknown argument 'x'"), "{err}");
        assert_eq!(
            header
                .validate_args(Some(&json!({"address": 1})))
                .unwrap_err(),
            "Argument 'address' must be a string"
        );
    }

    #[test]
    fn arguments_required_by_default() {
        // Without "optional": true, every argument is required.
        let header = header("; { \"arguments\": { \"a\": { \"type\": \"number\" } } }");
        assert_eq!(
            header.validate_args(None).unwrap_err(),
            "Missing required argument 'a'"
        );
        assert!(header.validate_args(Some(&json!({"a": "12."}))).is_ok());
    }

    #[test]
    fn optional_arguments() {
        // "optional": true: a call may omit the argument.
        let header =
            header("; { \"arguments\": { \"a\": { \"type\": \"number\", \"optional\": true } } }");
        assert!(header.validate_args(None).is_ok());
        assert!(header.validate_args(Some(&json!({"a": "12."}))).is_ok());
    }

    #[test]
    fn enum_only_argument() {
        let header = header("; { \"arguments\": { \"a\": { \"enum\": [\"R\", \"W\"] } } }");
        assert!(header.validate_args(Some(&json!({"a": "R"}))).is_ok());
        assert!(header.validate_args(Some(&json!({"a": "X"}))).is_err());
    }

    #[test]
    fn invalid_headers() {
        // Under strict, header problems reject the script.
        for (script, msg) in [
            // Unknown keywords anywhere are errors.
            (
                "; { \"interface\": 1, \"default\": 1 }",
                "unsupported keyword \"default\" in the header",
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"symbol\", \"description\": \"x\" } } }",
                "in argument 'a'",
            ),
            (
                "; { \"interface\": \"one\" }",
                "\"interface\" must be a number",
            ),
            (
                "; { \"strict\": \"yes\" }",
                "\"strict\" must be true or false",
            ),
            ("; { \"arguments\": [] }", "\"arguments\" is no JSON object"),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"color\" } } }",
                "unknown type \"color\"",
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"symbol\", \"enum\": [\"R\"] } } }",
                "exactly one of",
            ),
            ("; { \"arguments\": { \"a\": { } } }", "exactly one of"),
            (
                "; { \"arguments\": { \"a\": { \"pattern\": \"[A-Za-z]+\" } } }",
                "must start with ^",
            ),
            (
                "; { \"required\": [\"addr\"] }",
                "unsupported keyword \"required\" in the header",
            ),
            (
                "; { \"arguments\": { \"a\": { \"type\": \"symbol\", \"optional\": \"yes\" } } }",
                "\"optional\" of 'a' must be true or false",
            ),
            (
                "; { \"arguments\": { \"a\": { \"enum\": \"R\" } } }",
                "no list of strings",
            ),
        ] {
            let err = ScriptHeader::parse(script, STRICT).unwrap_err();
            assert!(err.contains(msg), "{msg}: {err}");
            // Under the other policies, the script runs anyway.
            let header = ScriptHeader::parse(script, UNRESTRICTED).unwrap();
            assert_eq!(header.warnings.len(), 1, "{script}: {:?}", header.warnings);
            assert!(
                header.warnings[0].contains(msg),
                "{script}: {:?}",
                header.warnings
            );
            assert!(header.warnings[0].contains("runs anyway"), "{script}");
        }
    }

    #[test]
    fn example_comment_not_parsed() {
        // The JSON after the header (e.g. an example) is convention only.
        let script = "; { \"interface\": 1 }\n;\n; Example:\n; { \"interface\": 99 }\n";
        let header = ScriptHeader::parse(script, UNRESTRICTED).unwrap();
        assert_eq!(header.interface, Some(1));
    }

    #[test]
    fn interface_versions() {
        // Skill scripts must declare "interface": under strict, every script
        // is a skill script.
        let err = ScriptHeader::parse("; No header at all.\n", STRICT).unwrap_err();
        assert!(
            err.contains("does not declare its t32mcp interface version"),
            "{err}"
        );
        // A demo or user script is not checked under the other policies.
        let header = ScriptHeader::parse("; No header at all.\n", UNRESTRICTED).unwrap();
        assert_eq!(header.interface, None);
        assert!(header.warnings.is_empty());
        // A too new interface needs a t32mcp upgrade: rejected under strict.
        let future = "; { \"interface\": 99, \"strict\": true }";
        let err = ScriptHeader::parse(future, STRICT).unwrap_err();
        assert!(err.contains("Upgrade t32mcp"), "{err}");
        // Under the other policies, the script runs anyway.
        let header = ScriptHeader::parse(future, UNRESTRICTED).unwrap();
        assert_eq!(header.warnings.len(), 1, "{:?}", header.warnings);
        assert!(
            header.warnings[0].contains("Upgrade t32mcp"),
            "{:?}",
            header.warnings
        );
        // With a single supported version, any other version is older.
        if SCRIPT_INTERFACE_VERSION > 0 {
            let older = format!(
                "; {{ \"interface\": {}, \"strict\": true }}",
                SCRIPT_INTERFACE_VERSION - 1
            );
            let err = ScriptHeader::parse(&older, STRICT).unwrap_err();
            assert!(err.contains("invalid t32mcp interface"), "{err}");
        }
    }

    #[test]
    fn validate_args() {
        let header = ScriptHeader::parse(SCRIPT, STRICT).unwrap();
        assert!(
            header
                .validate_args(Some(&json!({"address": "main", "access": "X"})))
                .is_ok()
        );
        // Missing, unknown or mismatching arguments reject the call.
        assert_eq!(
            header.validate_args(None).unwrap_err(),
            "Missing required argument 'address'"
        );
        assert!(
            header
                .validate_args(Some(&json!({"address": "main", "x": "1"})))
                .unwrap_err()
                .contains("Unknown argument 'x'")
        );
        // script_args must be an object.
        assert_eq!(
            header.validate_args(Some(&json!([]))).unwrap_err(),
            "script_args must be an object"
        );
        // A script without "arguments" takes no arguments.
        let header =
            ScriptHeader::parse("; { \"interface\": 1, \"strict\": true }", STRICT).unwrap();
        assert!(header.validate_args(None).is_ok());
        assert_eq!(
            header.validate_args(Some(&json!({"a": "b"}))).unwrap_err(),
            "This script takes no arguments: its header declares no arguments"
        );
    }
}
