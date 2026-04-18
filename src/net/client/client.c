#include "client.h"
#include "src/util/logger.h"

static Client client = {0};

int client_thread(void* arg) {
	ENetHost* host = arg;
	ENetEvent event;

	while (SDL_GetAtomicInt(&client.running)) {
		while (enet_host_service(host, &event, CLIENT_POLL_TIMEOUT_MS)) {
			switch (event.type) {
				case ENET_EVENT_TYPE_RECEIVE:
					NetPacket* packet = packet_create_input(event.packet->data, (u32)event.packet->dataLength);
					queue_push(&client.queue, packet);
				case ENET_EVENT_TYPE_CONNECT:
					break;
				case ENET_EVENT_TYPE_DISCONNECT:
					break;
				case ENET_EVENT_TYPE_NONE:
					log_warn("client thread recieve invalid event");
					break;
			}
		}
	}


	return 0;
}

b8 client_create(ClientCreateInfo info) {
	if (SDL_GetAtomicInt(&client.connected)) {
		log_warn("client already connected to a network");
		return false;
	}

	client.client = enet_host_create(
		NULL,
		1,
		CLIENT_MAX_CHANNELS,
		0,
		0
	);

	if (client.client == NULL) {
		log_err("client failed to initialize with enet");
		return false;
	}

	enet_address_set_host(client.address);
	client.address.port = info.server_port;
	client.address.host = info.server_address;

	client.peer = enet_host_connect(client.)
}
b8 client_destroy(void) {

}

void client_send_packet(NetPacket* packet) {

}


