import logging
import os
import secrets

import pytest
from fastmcp import Client
from fastmcp.client.transports import StdioTransport, StreamableHttpTransport
import socket
import subprocess
import time

LOGGER = logging.getLogger("mcp")
TESTDIR_NAME = "tests"

def get_testdir_path(request):
    path = []
    testdir_found = False
    for p in request.config.inipath.parts:
        path.append(p)
        if p == TESTDIR_NAME:
            testdir_found = True
            break
    assert testdir_found
    return path

def t32mcp_path(request):
    if "T32MCP" in os.environ:
        path_to_server = os.environ["T32MCP"].replace('"', "").replace("'", "")
    else:
        RELATIVE_PATH_TO_SERVER = (
            "../t32mcp-rs/target/debug/t32mcp"
            if os.name == "posix"
            else "../t32mcp-rs/target/debug/t32mcp.exe"
        )
        path = get_testdir_path(request)
        path.append(RELATIVE_PATH_TO_SERVER)
        path_to_server = os.path.join(*path)

    assert(os.path.isfile(path_to_server))
    return path_to_server

def skills_dir(request):
    """The repository's skills/ directory, a valid --skills root."""
    repo = os.path.dirname(os.path.join(*get_testdir_path(request)))
    skills = os.path.join(repo, "skills")
    assert os.path.isdir(skills)
    return skills

def free_port() -> int:
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]

def wait_for_port(port: int, process: subprocess.Popen, timeout: float = 10.0):
    """Wait until the server accepts connections, so that the client does not
    talk to nothing (or to another server) while t32mcp is still starting."""
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        assert process.poll() is None, f"t32mcp exited with {process.returncode}"
        try:
            with socket.create_connection(("127.0.0.1", port), timeout=0.2):
                return
        except OSError:
            time.sleep(0.05)
    raise TimeoutError(f"t32mcp does not listen on port {port}")

@pytest.fixture(scope="module")
def local_mcp_server(request) -> Client:
    path_to_server = t32mcp_path(request)
    LOGGER.info(f"Setting up STDIO server: {path_to_server}")
    transport = StdioTransport(command=path_to_server, args=["--execution-policy", "unrestricted"])
    client = Client(transport)
    return client

@pytest.fixture(scope="module")
def remote_mcp_server(request) -> Client:
    path_to_server = t32mcp_path(request)
    # T32MCP_HTTP_PORT sets a fixed port. Default: a free port.
    PORT = int(os.environ.get("T32MCP_HTTP_PORT", free_port()))
    TOKEN = secrets.token_urlsafe(32)
    LOGGER.info(f"Setting up HTTP server: {path_to_server} (port {PORT})")
    t32mcp = subprocess.Popen([path_to_server, "--http", str(PORT), "--token", TOKEN, "--execution-policy", "unrestricted"])
    wait_for_port(PORT, t32mcp)
    def terminate():
        LOGGER.info(f"Closing {path_to_server}")
        t32mcp.terminate()
        time.sleep(0.5)
    request.addfinalizer(lambda: terminate())

    transport = StreamableHttpTransport(url=f"http://127.0.0.1:{PORT}/mcp", auth=TOKEN)
    client = Client(transport)
    return client
