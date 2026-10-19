import os
import subprocess
import time

import httpx
import pytest

from conftest import free_port, skills_dir, t32mcp_path, wait_for_port

TOKEN = "test-token"
ALLOWED_ORIGIN = "http://localhost:6274"
INITIALIZE = {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "initialize",
    "params": {
        "protocolVersion": "2025-11-25",
        "capabilities": {},
        "clientInfo": {"name": "test_auth", "version": "0"},
    },
}

@pytest.fixture(scope="module")
def url(request):
    """HTTP server with the token from T32MCP_TOKEN (the fallback of --token)."""
    port = free_port()
    env = dict(os.environ, T32MCP_TOKEN=TOKEN)
    t32mcp = subprocess.Popen(
        [t32mcp_path(request), "--http", str(port), "--allow-origin", ALLOWED_ORIGIN + "/", "--execution-policy", "strict", "--port", "20000", "--skills", skills_dir(request)],
        env=env,
    )
    wait_for_port(port, t32mcp)
    yield f"http://127.0.0.1:{port}/mcp"
    t32mcp.terminate()
    time.sleep(0.5)

@pytest.mark.parametrize("headers", [
    {},
    {"Authorization": "Bearer wrong-token"},
    {"Authorization": TOKEN},
    {"Authorization": f"Basic {TOKEN}"},
])
def test_rejected(url, headers):
    response = httpx.post(url, json=INITIALIZE, headers=headers)
    assert response.status_code == 401
    assert response.headers["WWW-Authenticate"] == "Bearer"

def test_accepted(url):
    response = httpx.post(url, json=INITIALIZE, headers={"Authorization": f"Bearer {TOKEN}"})
    assert response.status_code == 200
    assert response.json()["result"]["serverInfo"]["name"]

@pytest.mark.parametrize("origin", ["http://evil.example", "http://localhost:6275", "null"])
def test_origin_rejected(url, origin):
    """Origins not given with --allow-origin are rejected, even with the token."""
    headers = {"Authorization": f"Bearer {TOKEN}", "Origin": origin}
    response = httpx.post(url, json=INITIALIZE, headers=headers)
    assert response.status_code == 403
    assert "Access-Control-Allow-Origin" not in response.headers

def test_origin_allowed(url):
    headers = {"Authorization": f"Bearer {TOKEN}", "Origin": ALLOWED_ORIGIN}
    response = httpx.post(url, json=INITIALIZE, headers=headers)
    assert response.status_code == 200
    assert response.headers["Access-Control-Allow-Origin"] == ALLOWED_ORIGIN

def test_origin_allowed_needs_token(url):
    response = httpx.post(url, json=INITIALIZE, headers={"Origin": ALLOWED_ORIGIN})
    assert response.status_code == 401
    assert response.headers["Access-Control-Allow-Origin"] == ALLOWED_ORIGIN

def test_preflight(url):
    """The CORS preflight of a browser carries no token."""
    headers = {
        "Origin": ALLOWED_ORIGIN,
        "Access-Control-Request-Method": "POST",
        "Access-Control-Request-Headers": "authorization, content-type",
    }
    response = httpx.options(url, headers=headers)
    assert response.status_code == 204
    assert response.headers["Access-Control-Allow-Origin"] == ALLOWED_ORIGIN
    assert response.headers["Access-Control-Allow-Methods"] == "POST"
    assert response.headers["Access-Control-Allow-Headers"] == "authorization, content-type"

def test_preflight_rejected(url):
    headers = {"Origin": "http://evil.example", "Access-Control-Request-Method": "POST"}
    assert httpx.options(url, headers=headers).status_code == 403
