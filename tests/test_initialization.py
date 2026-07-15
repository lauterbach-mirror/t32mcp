import logging
from fastmcp import Client
from mcp.types import Tool

LOGGER = logging.getLogger("mcp")

async  def test_server_connection(local_mcp_server: Client, remote_mcp_server: Client):
    def print_server_info(info, debug=False):
        log = LOGGER.debug if debug else LOGGER.info
        log(f"{info.name} (v{info.version})")
        log(f"{info.title}")
        log(f"{info.websiteUrl}")

    async with local_mcp_server:
        LOGGER.info("Connected to local server")
        local_info = local_mcp_server.initialize_result.serverInfo
        print_server_info(local_info, debug=True)

    async with remote_mcp_server:
        LOGGER.info("Connected to remote server")
        remote_info = remote_mcp_server.initialize_result.serverInfo
        print_server_info(remote_info, debug=True)

    assert(local_info == remote_info)
    print_server_info(local_info)

async def test_list_tools(local_mcp_server: Client, remote_mcp_server: Client):
    local_tools: list[Tool] = []
    remote_tools: list[Tool] = []

    async with local_mcp_server:
        LOGGER.info("Connected to local server")
        server = local_mcp_server
        tools = await server.list_tools()
        for tool in tools:
            LOGGER.debug(f"Tool detected: {tool.name}")
            local_tools.append(tool)

    async with remote_mcp_server:
        LOGGER.info("Connected to remote server")
        server = remote_mcp_server
        tools = await server.list_tools()
        for tool in tools:
            LOGGER.debug(f"Tool detected: {tool.name}")
            remote_tools.append(tool)

    assert(len(local_tools) == len(remote_tools))
    local_tools.sort(key=lambda t: t.description or "")
    remote_tools.sort(key=lambda t: t.description or "")
    for (tool, duplicate) in zip(local_tools, remote_tools):
        assert(tool == duplicate)
        LOGGER.info(f"Tool: {tool.name}")
