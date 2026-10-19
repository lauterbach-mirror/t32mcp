The scripts come from the lauterbach-* agent skills. If they are not
installed or outdated: the tool trace32_install_skills of Lauterbach's t32chat
MCP server ({SOURCE}/mcp, login required) returns the install command; run it
with a local skills directory of the agent (skill lauterbach-t32chat) and tell
the user to restart the session. It replaces all lauterbach-* skills: local
changes to them are lost. This server runs scripts by absolute path on its
host, so the skills must be local files. Without the t32chat MCP server, ask
the user to install them: {PAGE}