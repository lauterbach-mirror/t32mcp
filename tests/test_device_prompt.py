"""device_prompt of trace32_run_script and trace32_run_command. Needs a PowerView."""

from fastmcp import Client
from fastmcp.client.transports import StdioTransport

from conftest import t32mcp_path
from test_contract import T32_PORT, pytestmark  # noqa: F401 (skip without PowerView)

# AREA.view works under every device prompt. WINdow.COMMAND() shows the prompt
# that the window was opened with.
PROBE = """WinPOS ,,,,,, DPTEST
AREA.view
PRINT WINdow.COMMAND(DPTEST)
WinCLEAR DPTEST"""


async def run_command(client: Client, **args):
    result = await client.call_tool("trace32_run_command", {"command": PROBE, **args}, raise_on_error=False)
    return result.is_error, result.content[0].text


async def test_device_prompt(request):
    client = Client(StdioTransport(command=t32mcp_path(request), args=["--port", str(T32_PORT), "--execution-policy", "unrestricted"]))
    async with client:
        # Selects "::" permanently, like a user typing it in the command line.
        is_error, text = await run_command(client, device_prompt="::")
        assert not is_error, text
        assert "::AREA.view" in text and "B::AREA.view" not in text, text
        # The default selects B:: again.
        is_error, text = await run_command(client)
        assert not is_error, text
        assert "B::AREA.view" in text, text


async def test_device_prompt_rejected(request):
    client = Client(StdioTransport(command=t32mcp_path(request), args=["--port", str(T32_PORT), "--execution-policy", "unrestricted"]))
    async with client:
        for prompt in ["B:: DO x.cmm", "B:", "B::\nDO x.cmm", ""]:
            is_error, text = await run_command(client, device_prompt=prompt)
            assert is_error, prompt
            assert "device_prompt must be" in text, text
