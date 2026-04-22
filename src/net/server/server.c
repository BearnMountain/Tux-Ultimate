#include "server.h"
#include "src/util/logger.h"

static Server server = {0};

typedef struct {
	u32 id;
	char* username;
} ClientData;

// helpers
void broadcast_packet(ENetHost* server, NetPacket* data);
void send_direct_packet(ENetPeer* peer, NetPacket* data);



int threaded_server(void* data) {
	ENetHost* host = data;
	ENetEvent event;

	while (SDL_GetAtomicInt(&server.running)) {
		while (enet_host_service(host, &event, SERVER_POLL_TIMEOUT_MS) > 0) {
			switch (event.type) {
				case ENET_EVENT_TYPE_CONNECT:
					char ip[64];
					enet_address_get_host_ip(&event.peer->address, ip, sizeof(ip));
					i32 slot = -1;
					for (i32 i = 0; i < (i32)server.max_clients; i++)
						if (server.connected_clients[i] == NULL)
							slot = i;

					if (slot < 0) {
						log_warn("client %s tried to connect but server is full", ip);
						enet_peer_reset(event.peer);
						break;
					}

					// loading client into server info
					server.connected_clients[slot] = event.peer;
					event.peer->data = (void*)(uintptr_t)slot;
					log_info("client connected from %s (slot %d)", ip, slot);

					break;
				case ENET_EVENT_TYPE_DISCONNECT:
					for (i32 i = 0; i < (i32)server.max_clients; i++) {
						if (event.peer == server.connected_clients[i]) {
							log_info("client disconnected (slot %d)", i);
							server.connected_clients[slot] = NULL;
						}
					}
					event.peer->data = NULL;

					break;
				case ENET_EVENT_TYPE_RECEIVE: 
					NetPacket* packet = packet_deserialize(event.packet, true);
					if (packet) {
						queue_push(&server.queue, packet);
						log_info("recieved packet");
					}
					break;
					
				case ENET_EVENT_TYPE_NONE:
					log_warn("server error with event");
					break;
			}
		}

		// server operations
		NetPacket* packet = queue_pop(&server.queue);
		if (packet) {
			broadcast_packet(host, packet);
			log_info("sending packet out");
		}
	}

	// stops running do some cleanup
	log_info("Shutting down server");
	for (u32 i = 0; i < server.max_clients; i++) {
		if (server.connected_clients[i]) {
			enet_peer_disconnect(server.connected_clients[i], 0);
			server.connected_clients[i] = NULL;
		}
	}

	enet_host_service(host, &event, 100);
	enet_host_destroy(host);
	log_info("server thread shutting down");

	enet_host_destroy(host);

	return 0;
}



void broadcast_packet(ENetHost* server, NetPacket* data) {
	ENetPacket* packet = packet_create_enet(data, true);
	enet_host_broadcast(server, 0, packet);
	enet_host_flush(server);
}

void send_direct_packet(ENetPeer* peer, NetPacket* data) {
	ENetPacket* packet = packet_create_enet(data, true);
	if (enet_peer_send(peer, 0, packet) < 0) {
		log_err("failed to send packet to peer");
		enet_packet_destroy(packet);
	}
}


b8 server_start(ServerCreateInfo info) {
	if (SDL_GetAtomicInt(&server.running)) {
		log_warn("network already initialized");
		return 0;
	}

	if (info.max_clients > SERVER_MAX_CLIENTS) {
		log_warn("out of network bounds, max clients: %d, submited: %d", SERVER_MAX_CLIENTS, info.max_clients);
		return 0;
	}

	server.address.host = ENET_HOST_ANY;
	server.address.port = info.port;

	ENetHost* host = enet_host_create(
		&server.address,
		server.max_clients,
		SERVER_MAX_CHANNELS,
		0,
		0
	);

	if (host == NULL) {
		log_err("failed to start server");
		return false;
	}

	// intialize server
	queue_init(&server.queue);
	SDL_SetAtomicInt(&server.running, 1);
	server.max_clients = info.max_clients;
	for (u32 i = 0; i < server.max_clients; i++) {
		server.connected_clients[i] = NULL;
	}

	server.thread = SDL_CreateThread(threaded_server, "TuxUltimateServer", (void*)host);

	return true;
}

b8 server_stop(void) {
	if (!SDL_GetAtomicInt(&server.running)) {
		log_warn("cant stop unitialized server");
		return false;
	}
	// server should clean itself up on its own thread, only running should 
	// be accessible from outside
	SDL_SetAtomicInt(&server.running, false);

	return true;
}
