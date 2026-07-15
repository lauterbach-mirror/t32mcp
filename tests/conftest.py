import logging
import os

import pytest
from fastmcp import Client
from fastmcp.client.transports import StdioTransport, StreamableHttpTransport
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

    assert(os.path.exists(path_to_server))
    return path_to_server

@pytest.fixture(scope="module")
def local_mcp_server(request) -> Client:
    path_to_server = t32mcp_path(request)
    LOGGER.info(f"Setting up STDIO server: {path_to_server}")
    transport = StdioTransport(command=path_to_server, args=[])
    client = Client(transport)
    return client

@pytest.fixture(scope="module")
def remote_mcp_server(request) -> Client:
    path_to_server = t32mcp_path(request)
    PORT = 8000
    LOGGER.info(f"Setting up HTTP server: {path_to_server} (port {PORT})")
    cmd_line = f"{path_to_server} --http {PORT}"
    t32mcp = subprocess.Popen(cmd_line.split())
    time.sleep(1)
    def terminate():
        LOGGER.info(f"Closing {path_to_server}")
        t32mcp.terminate()
        time.sleep(0.5)
    request.addfinalizer(lambda: terminate())

    transport = StreamableHttpTransport(url=f"http://127.0.0.1:{PORT}/mcp")
    client = Client(transport)
    return client
