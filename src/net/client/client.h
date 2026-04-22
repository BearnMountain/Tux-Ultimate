#ifndef CLIENT_H_
#define CLIENT_H_

#include <enet/enet.h>
#include "src/net/shared/packet_queue.h"
#include "src/util/defines.h"
#include "src/net/shared/packet.h"
#include <SDL3/SDL_atomic.h>
#include <SDL3/SDL_thread.h>

#define CLIENT_POLL_TIMEOUT_MS 100
#define CLIENT_MAX_CHANNELS 2

typedef struct {
	// thread info 
	PacketQueue queue;

	// connecting to network
	SDL_AtomicInt connected;
	ENetAddress address;
	ENetHost* net_manager;
	ENetPeer* server;

	SDL_Thread* thread;
} Client;

typedef struct {
	u16 port;
	const char* host_name;
	const char* player_name;
} ClientCreateInfo;

b8 client_connect(ClientCreateInfo info);
b8 client_disconnect(void);

// TCP packet sending
void client_send_packet(NetPacket* packet, b8 cleanup);
NetPacket* client_poll_packet(void);

#endif
