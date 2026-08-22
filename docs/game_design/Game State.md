# Game State
This game runs on an authoritative server model:
- Client connects to a server with metadata:
    - character selection, username, etc
- Client packets are serialized then sent to the server:
    - keyinputs and mouse motion/clicks/buttons/location
    - ui decisions
- Server deserializes packets for a queue
- Server applies physics calculations + collision remediation 
  to update players
- Finalized packet is 'tick' stamped and served to all clients

# Game Tick
Authoritative server model requires that all clients are 
synced to server tick rate
- Each frame, takes most up to date packet(udp means has to 
  sort metadata first because packets out of order)

# Server Features
All servers are hosted by the community.
## Base Features
Players vote on maps and select their own characters.
Different gamemodes provide more enjoyment of the game. TO BE DETERMINED
## Self Hosting
Player self hosts a server, which is threaded to ensure own processing 
of information and to offload large volume of work as another computer
process. 
Host is provided large range of modes and control over the game.
Host must share their own IP to allow friends or community to connect.
## Headless
Computer hosts the server but doesnt make a client side interface:
- Self hosting provides client interface where they can play in the
  match themself
- Headless just provides control over the server
- Headless can also provide a "broadcast" like experience for onlookers
    - i.e. client's model is centered, headless encapsulates all 
      connected clients character models but can switch between different 
      perspectives
