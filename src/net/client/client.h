#ifndef CLIENT_H_
#define CLIENT_H_

#include <enet/enet.h>
#include "src/net/shared/packet_queue.h"
#include "src/util/defines.h"
#include "src/net/shared/packet.h"
#include <SDL3/SDL_atomic.h>

#define CLIENT_POLL_TIMEOUT_MS 100
#define CLIENT_MAX_CHANNELS 2

typedef struct {
	// thread info 
	SDL_AtomicInt running;	// client thread for listening for packets, decompressing them and storing them in a queue
	PacketQueue queue;

	// connecting to network
	SDL_AtomicInt connected;
	ENetAddress address;
	ENetPeer* peer;
} Client;

typedef struct {
	ENetAddress server_address;
	u32 server_port;
	char* player_name;
} ClientCreateInfo;

b8 client_create(ClientCreateInfo info);
b8 client_destroy(void);

// TCP packet sending
void client_send_packet(NetPacket* packet);

#endif
