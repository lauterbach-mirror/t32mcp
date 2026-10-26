"""Contract tests of the script interface, see README.md. Need a PowerView."""

from pathlib import Path

import pytest
from fastmcp import Client
from fastmcp.client.transports import StdioTransport

from conftest import T32_PORT, t32mcp_path

CONTRACT = Path(__file__).parent / "contract"

# A PowerView on T32_PORT, started by the fixture if needed; skipped without one.
pytestmark = pytest.mark.usefixtures("powerview")


async def run(request, interface: int, script: str, args: dict | None = None):
    """The contract skill tests/contract/interface-<n>/contract, loaded via --skills."""
    client = Client(
        StdioTransport(
            command=t32mcp_path(request),
            args=["--port", str(T32_PORT), "--execution-policy", "strict", "--skills", str(CONTRACT / f"interface-{interface}")],
        )
    )
    async with client:
        result = await client.call_tool(
            "trace32_run_script",
            {"skill_name": "contract", "script_name": script, "script_args": args or {}, "timeout": 10},
            raise_on_error=False,
        )
    return result.is_error, result.content[0].text


@pytest.mark.parametrize(
    "value",
    ["hello world", "key=value inside", "C:/path with space/file.cmm", "Ünïcödé", ""],
)
async def test_interface_1_arguments(request, value):
    is_error, text = await run(request, 1, "args.cmm", {"text": value, "number": "41."})
    assert not is_error, text
    assert f"text=[{value}]" in text
    assert "number=42" in text.splitlines()


async def test_interface_1_output(request):
    is_error, text = await run(request, 1, "output.cmm")
    assert not is_error, text
    lines = [line for line in text.splitlines() if line.strip()]
    for expected in ["plain line", "warning line", "error line", "last line"]:
        assert expected in lines, text
    assert lines.index("plain line") < lines.index("last line")


async def test_interface_1_error(request):
    is_error, text = await run(request, 1, "error.cmm")
    assert is_error, text
    assert "before the error" in text
    assert "NoSuchCommand.Really" in text
    assert "after the error" not in text


async def test_interface_1_caught_error(request):
    """The error handler of the script prints the failure marker: a failed result."""
    is_error, text = await run(request, 1, "caught_error.cmm")
    assert is_error, text
    assert "before the error" in text
    assert "NoSuchCommand.Really" in text
    assert "[MCP end]" in text
    assert "after the error" not in text
