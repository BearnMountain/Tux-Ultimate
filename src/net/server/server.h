#ifndef SERVER_H_
#define SERVER_H_

#define SERVER_MAX_CLIENTS 8
#define SERVER_MAX_CHANNELS 2
#define SERVER_PORT 2760
#define SERVER_POLL_TIMEOUT_MS 100

#include <enet/enet.h>
#include <SDL3/SDL_atomic.h>
#include <SDL3/SDL_thread.h>
#include "src/util/defines.h"

/*
Network Redundancy: 32 packets of history
Network puts packets on queue
Simulation/Physics Engine removes packets from queue

Network Layout
- Different Channels for different data
    - Input Channel: data on buttons pressed, unreliable
    - State Channel: current snapshot of all inputs
        - https://www.ggpo.net/ could help to reduce lag, used in other fighting games
        - authoritative states sent(interpolated by client to reduce visual issues)
            - packets hold position, velocity
    - Event Channel: small channel for information like chat, ui, etc. reliable because
    its useful info, but not time sensitive
    - History Buffer: rollback for previous/future states so sent data can be delivered to
    other used before they reach that visual state in game
        - lag comes with this, might have render set back 100ms so packet can be recieved by
        other player so you and the other client are updated at once
- Have a tick rate for number of packets sent, also data for packets to see if unreliable packet
should be trashed if old data
- Security: libsodium useful for protecting data to prevent cheating
- Network recieves data -> queue and is processed by some engine -> back to all users
- Client queues data from server -> buffer of packets -> game engine only renders first
half of the buffer so it has scenes to render without waiting for server

client -> server : player inputs
server -> client : gamestate snapshot(position, health, )
*/

typedef struct {
	// enet required info
	ENetAddress address;
	ENetHost* server;

	// server thread 
	SDL_Thread* thread;
	SDL_AtomicInt running;

	// client info
	u32 max_clients; // can set max to <= 8
	ENetPeer* connected_clients[SERVER_MAX_CLIENTS];

	// storage of incoming packets
	// PacketQueue queue;

	// server info
	u32 tick;
} Server;

typedef struct {
	u32 port;
	u32 max_clients;
} ServerCreateInfo;

// only one instance per process
// - creates threaded server
b8 server_create(ServerCreateInfo info);
b8 server_destroy(void);

#endif
