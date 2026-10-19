import os
import subprocess

import pytest
from fastmcp import Client
from fastmcp.client.transports import StdioTransport

from conftest import t32mcp_path


def policy_client(request, *args: str) -> Client:
    return Client(StdioTransport(command=t32mcp_path(request), args=list(args)))


def strict_args(skill_tree, *args: str) -> tuple:
    """Server arguments of the strict policy: it only runs skill scripts
    inside the --skills directories, so it requires one."""
    return ("--execution-policy", "strict", "--port", "20000", "--skills", str(skill_tree / "root"), *args)


SCHEMA = """; {
;   "interface": 1,
;   "strict": true,
;   "arguments": {
;     "address": { "type": "address" },
;     "note": { "type": "string", "optional": true }
;   }
; }

ENDDO
"""


@pytest.fixture
def skill_tree(tmp_path):
    """<tmp>/root/my-skill/{SKILL.md, scripts/*.cmm} and <tmp>/other/b.cmm.

    a.cmm: interface, but no "strict" opt-in and no schema.
    schema.cmm: "strict": true, "address" (type address) required, "note" optional.
    unstrict.cmm: like schema.cmm, but without the "strict" opt-in.
    nointerface.cmm, future.cmm: no or a too new interface version.
    """
    scripts = tmp_path / "root" / "my-skill" / "scripts"
    scripts.mkdir(parents=True)
    (tmp_path / "root" / "my-skill" / "SKILL.md").write_text("")
    (scripts / "a.cmm").write_text('; { "interface": 1 }\n\nENDDO\n')
    (scripts / "schema.cmm").write_text(SCHEMA)
    (scripts / "unstrict.cmm").write_text(SCHEMA.replace('  "strict": true,', ""))
    (scripts / "nointerface.cmm").write_text("; Some script.\n\nENDDO\n")
    (scripts / "future.cmm").write_text('; { "interface": 99 }\n\nENDDO\n')
    (tmp_path / "other").mkdir()
    (tmp_path / "other" / "b.cmm").write_text("")
    return tmp_path


async def call_script(client: Client, script_args: dict | None = None, **address):
    """Address a script by script_path, or (under strict) by skill_name and script_name."""
    arguments = dict(address)
    if script_args is not None:
        arguments["script_args"] = script_args
    async with client:
        return await client.call_tool("trace32_run_script", arguments, raise_on_error=False)


@pytest.mark.parametrize(
    "policies",
    [
        ["scripts"],
        ["strict"],
        # The most restrictive policy wins.
        ["strict", "unrestricted"],
    ],
)
async def test_run_command_hidden(request, skill_tree, policies):
    args = []
    for policy in policies:
        args += ["--execution-policy", policy]
    if "strict" in policies:
        args += ["--port", "20000", "--skills", str(skill_tree / "root")]
    async with policy_client(request, *args) as client:
        names = [t.name for t in await client.list_tools()]
    assert "trace32_run_command" not in names


def test_strict_needs_skills(request):
    """Strict resolves scripts inside the --skills directories:
    without one, the server refuses to start."""
    process = subprocess.run(
        [t32mcp_path(request), "--execution-policy", "strict", "--port", "20000", "--skills", str(skill_tree)],
        stdin=subprocess.DEVNULL, capture_output=True, text=True, timeout=10,
    )
    assert process.returncode != 0
    assert "--skills" in process.stderr


async def test_strict_described(request, skill_tree):
    async with policy_client(request, *strict_args(skill_tree)) as client:
        instructions = client.initialize_result.instructions
        tools = {t.name: t for t in await client.list_tools()}
    assert "lauterbach-t32mcp-execution-policy" in instructions
    assert "lauterbach-t32mcp-execution-policy" in tools["trace32_run_script"].description


async def test_strict_rejects_script_path(request, skill_tree):
    """Under strict, scripts are addressed by skill_name and script_name inside
    the --skills directories, never by a path (which could lead outside them)."""
    client = policy_client(request, *strict_args(skill_tree))
    for path in [str(skill_tree / "other" / "b.cmm"), "~~/demo/foo.cmm"]:
        result = await call_script(client, script_path=path)
        assert result.is_error
        assert "missing skill_name" in result.content[0].text


async def test_skills_rejects_skills_elsewhere(request, skill_tree):
    """Strict resolves skills only inside the --skills directories:
    a skill that exists elsewhere on disk is not found."""
    client = policy_client(
        request, "--execution-policy", "strict", "--skills", str(skill_tree / "other"), "--port", "20000"
    )
    result = await call_script(client, skill_name="my-skill", script_name="schema.cmm")
    assert result.is_error
    assert "Skill 'my-skill' not found in the --skills directories" in result.content[0].text


async def test_skill_script_passes_policy(request, skill_tree):
    # Not rejected: the call goes on to PowerView (which may not run here).
    client = policy_client(
        request, *strict_args(skill_tree),
        "--port", os.environ.get("T32MCP_UNUSED_PORT", "1"),
    )
    result = await call_script(
        client, skill_name="my-skill", script_name="schema.cmm", script_args={"address": "A:main"}
    )
    assert "Rejected by execution policy" not in result.content[0].text


def script(skill_tree, name: str) -> str:
    return str(skill_tree / "root" / "my-skill" / "scripts" / name)


async def test_strict_requires_opt_in(request, skill_tree):
    # "strict": true is an opt-in: scripts without it are rejected under strict.
    client = policy_client(request, *strict_args(skill_tree))
    for name in ["a.cmm", "unstrict.cmm"]:
        result = await call_script(client, skill_name="my-skill", script_name=name)
        assert result.is_error, name
        assert '"strict": true' in result.content[0].text, (name, result.content[0].text)


# strict: header problems reject the call
@pytest.mark.parametrize(
    "name, args, error",
    [
        ("schema.cmm", {"address": 'A:main /CMD "DO x.cmm"'}, "does not match the pattern"),
        ("schema.cmm", {"address": "main", "x": "1"}, "Unknown argument 'x'"),
        ("schema.cmm", {"note": "hi"}, "Missing required argument 'address'"),
        ("nointerface.cmm", {}, "does not declare its t32mcp interface version"),
        ("future.cmm", {}, "Upgrade t32mcp"),
    ],
)
async def test_header_problems_rejected_under_strict(request, skill_tree, name, args, error):
    client = policy_client(request, *strict_args(skill_tree))
    result = await call_script(client, skill_name="my-skill", script_name=name, script_args=args)
    text = result.content[0].text
    assert result.is_error
    assert error in text
    assert "runs anyway" not in text


async def test_mismatch_message_contains_regex(request, skill_tree):
    client = policy_client(request, *strict_args(skill_tree))
    result = await call_script(
        client, skill_name="my-skill", script_name="schema.cmm", script_args={"address": "A:main /CMD"}
    )
    assert result.is_error
    assert "^[A-Za-z0-9_:.+\\-" in result.content[0].text


# argument problems reject the call under every policy
@pytest.mark.parametrize("policy", ["unrestricted", "scripts"])
@pytest.mark.parametrize(
    "name, args, error",
    [
        ("schema.cmm", {"address": 'A:main /CMD "DO x.cmm"'}, "does not match the pattern"),
        ("schema.cmm", {"address": "main", "x": "1"}, "Unknown argument 'x'"),
        ("schema.cmm", {"note": "hi"}, "Missing required argument 'address'"),
        # a script without schema takes no arguments
        ("a.cmm", {"anything": "goes"}, "takes no arguments"),
        ("unstrict.cmm", {"address": "main", "x": "1"}, "Unknown argument 'x'"),
    ],
)
async def test_argument_problems_rejected_everywhere(request, skill_tree, policy, name, args, error):
    client = policy_client(request, "--execution-policy", policy)
    result = await call_script(client, script_path=script(skill_tree, name), script_args=args)
    text = result.content[0].text
    assert result.is_error, text
    assert error in text
    assert "Warning" not in text


# unrestricted, scripts: other header problems are warnings, the script runs anyway
@pytest.mark.parametrize("policy", ["unrestricted", "scripts"])
@pytest.mark.parametrize(
    "name, warning",
    [
        # A missing interface declaration is only a problem under strict
        # (see test_header_problems_rejected_under_strict): outside strict,
        # the script may as well be a demo script.
        ("future.cmm", "Upgrade t32mcp"),
    ],
)
async def test_header_problems_warn_otherwise(request, skill_tree, policy, name, warning):
    # the call goes on to PowerView (not running here: connection error, with the warnings)
    client = policy_client(request, "--execution-policy", policy, "--port", "1")
    result = await call_script(client, script_path=script(skill_tree, name))
    text = result.content[0].text
    assert f"Warning: " in text and warning in text, text
    assert "runs anyway" in text
    assert "Rejected" not in text


async def test_interface_not_required_outside_strict(request, skill_tree):
    # A missing interface declaration is not checked outside strict: the
    # script may as well be a demo script.
    client = policy_client(request, "--execution-policy", "unrestricted", "--port", "1")
    for path in [str(skill_tree / "other" / "b.cmm"), script(skill_tree, "nointerface.cmm")]:
        result = await call_script(client, script_path=path)
        assert "Version mismatch" not in result.content[0].text


@pytest.mark.parametrize(
    "policy, name, args",
    [
        ("unrestricted", "a.cmm", {}),
        ("unrestricted", "schema.cmm", {"address": "main"}),
        ("scripts", "unstrict.cmm", {"address": "A:main", "note": "x"}),
        ("strict", "schema.cmm", {"address": "A:main"}),
    ],
)
async def test_arguments_accepted(request, skill_tree, policy, name, args):
    # Not rejected: the call goes on to PowerView (which does not run here).
    server_args = ["--execution-policy", policy, "--skills", str(skill_tree)]
    if policy == "strict":
        server_args += ["--skills", str(skill_tree / "root")]
        address = {"skill_name": "my-skill", "script_name": name}
    else:
        address = {"script_path": script(skill_tree, name)}
    client = policy_client(request, *server_args, "--port", "1")
    result = await call_script(client, script_args=args, **address)
    text = result.content[0].text
    assert "Rejected" not in text and "argument" not in text and "Invalid script header" not in text, text
