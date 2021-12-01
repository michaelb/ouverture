# Server-client model

ouverture works as a server-client application. The server is started (if not already running) by the first client invocation, and further calls from clients are converted to messages passed to the server.


## Project structure

The server is ouverture-core. It provides functions such as start(), stop(), and replies to queries

ouverture-cli and ouverture-ui are two different clients that can connect to the same server.
