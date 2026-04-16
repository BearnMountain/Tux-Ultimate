#include "server.h"

static Server server = {0}:

void* threaded_server() {


	return NULL;
}

b8 server_create(ServerCreateInfo info) {
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

	server.server = enet_host_create(
		&server.address,
		server.max_clients,
		SERVER_MAX_CHANNELS,
		0,
		0
	);

	if (server.server == NULL) {

	}

	// intialize server
	SDL_SetAtomicInt(&server.running = 1);
	server.max_clients = info.max_clients;
	for (int i = 0; i < server.max_clients; i++) {
		server.connected_clients[i] = NULL;
	}



	return true;
}
