# Between

A really small and simple MITM proxy for inspecting TCP traffic.
Exposes a Websocket server on which all traffic (bidirectional) is published in JSON format:

````json
{
    // Did the TCP "server" or client send this data
    "source": "client/server",

    // Unix timestamp of when data was transmitted
    "at": 1234567,

    "data": "<b64 encoded data>"
}
````

everything is configured in the `between.toml`:

````toml
[network]
# Address to accept WS connections on
ws_address = "0.0.0.0:8000"

# Upstream TCP server
proxy_listen_address = "0.0.0.0:8081"

# Downstream TCP client
proxy_connect_address = "127.0.0.1:8080"
````