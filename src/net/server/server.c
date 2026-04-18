#include "server.h"
#include "src/util/logger.h"

static Server server = {0};

int threaded_server(void* data) {
	ENetHost* s = data;
	ENetEvent event;

	while (SDL_GetAtomicInt(&server.running)) {
		while (enet_host_service(s, &event, SERVER_POLL_TIMEOUT_MS) > 0) {
			switch (event.type) {
				case ENET_EVENT_TYPE_RECEIVE: 
					ENetPacket* packet = event.packet;
					NetPacket* net_pack = packet_create_input((char*)packet->data, (u32)packet->dataLength);
					queue_push(&server.queue, net_pack);
					break;
				case ENET_EVENT_TYPE_CONNECT:
				case ENET_EVENT_TYPE_DISCONNECT:
				case ENET_EVENT_TYPE_NONE:
					log_warn("server error with event");
					break;
			}
		}
	}

	return 0;
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

	if (server.server == NULL) {
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


	return true;
}
