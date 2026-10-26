import logging
import os
import platform
import secrets

import pytest
from fastmcp import Client
from fastmcp.client.transports import StdioTransport, StreamableHttpTransport
import socket
import subprocess
import sys
import time

LOGGER = logging.getLogger("mcp")
TESTDIR_NAME = "tests"

# Remote API port of the PowerView for the tests that need one.
T32_PORT = int(os.environ.get("T32MCP_T32_PORT", "20000"))

def powerview_executable(t32sys: str) -> str:
    """The RISC-V simulator of the TRACE32 installation for this platform; keep
    in sync with ci/start_sim.sh."""
    if sys.platform == "win32":
        path = ["windows64", "t32mriscv.exe"]
    elif sys.platform == "darwin":
        path = ["macosx64", "t32mriscv-qt"]
    elif platform.machine().lower() in ("aarch64", "arm64"):
        path = ["linux-aarch64", "t32mriscv"]
    else:
        path = ["pc_linux64", "t32mriscv"]
    return os.path.join(t32sys, "bin", *path)

# Headless Instruction Set Simulator, configured by T32SYS and T32MCP_T32_PORT.
POWERVIEW_CONFIG = os.path.join(os.path.dirname(__file__), "sim.t32")

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
        assert process.poll() is None, f"{process.args[0]} exited with {process.returncode}"
        try:
            with socket.create_connection(("127.0.0.1", port), timeout=0.2):
                return
        except OSError:
            time.sleep(0.05)
    raise TimeoutError(f"{process.args[0]} does not listen on port {port}")

def powerview_reachable() -> bool:
    with socket.socket() as s:
        s.settimeout(0.5)
        return s.connect_ex(("127.0.0.1", T32_PORT)) == 0

@pytest.fixture(scope="session")
def powerview(tmp_path_factory):
    """A PowerView on T32_PORT: the one already running there, or an Instruction
    Set Simulator started from the TRACE32 installation in T32SYS for this
    session. Skips the test if there is neither."""
    if powerview_reachable():
        yield
        return
    t32sys = os.environ.get("T32SYS")
    if not t32sys:
        pytest.skip(f"no PowerView on port {T32_PORT} (T32MCP_T32_PORT) and no T32SYS to start one")
    executable = powerview_executable(t32sys)
    env = dict(os.environ, T32MCP_T32_PORT=str(T32_PORT))
    # Output to a log file, not DEVNULL: on Windows, PowerView crashes at start
    # when its stdout is the NUL device.
    log = tmp_path_factory.mktemp("powerview") / "powerview.log"
    LOGGER.info(f"Starting PowerView: {executable} (port {T32_PORT}, output: {log})")
    log_file = open(log, "w")
    process = subprocess.Popen(
        [executable, "-c", POWERVIEW_CONFIG], env=env, stdout=log_file, stderr=subprocess.STDOUT
    )
    try:
        wait_for_port(T32_PORT, process, timeout=30.0)
        yield
    finally:
        LOGGER.info("Closing PowerView")
        process.terminate()
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            process.kill()
        log_file.close()

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
