Manage the TRACE32 PowerView instances this server talks to, e.g. for AMP.
The instances given with --port are added at start (default: 20000).

action:
    - inspect: list the instances.
    - add: add the instance with Remote API TCP port 'port'. Fails if an
    instance with this port is already added. The instance may still be
    starting: it is connected lazily on the next request.
    - remove: remove the instance with port 'port'. Its AREA output pipe is
    detached first. PowerView keeps running.

Returns all instances in order: 'port' and 'state' (connected or not
reachable, with 'error'). Without port, the other tools use the first
reachable instance in this order. trace32_get_status shows their cores.
Several instances need different ports. A new instance gets its port from its
config file (RCL=NETTCP), the command line (--t32-api-rcl=TCP:<port>), or
`SETUP.API.RCL.Enable /PRotocol TCP /Port <port>.` entered in that instance
(the dot is needed for decimal numbers).