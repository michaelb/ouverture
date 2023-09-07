# Server-client model

ouverture works as a server-client application. The server is started (if not already running) by the first client invocation, and further calls from clients are converted to messages passed to the server.


## Project structure

The server is ouverture-server (ouverture_core is the lib, -server is lib+small main). It provides functions such as start(), stop(), and replies to queries

ouverture-cli and ouverture-ui ('ouverture' binary) are two different clients that can connect to the same server.

### ouverture-server/ouverture-ui open-close behavior:

Configuration provides 2 flags to control how ouverture ui/server interact with each other: 'external' and 'background'

- external = true : the ouverture server is supposed already running
- external = false : the ouverture server will be started as a UI's process child

- background = false : the server will stay 'in the foreground' and close when its launcher (terminal or UI) finishes/interrupts
- background = true : the server will be forked to the background and will stay alive no matter what happens

Note: if external = false and background = true (possibly the default), the UI should try to connect to an existing server _before_ launching a new one (To be implemented)

