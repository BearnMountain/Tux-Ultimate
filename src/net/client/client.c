#include "client.h"
#include "src/util/logger.h"
#include <SDL3/SDL_timer.h>
#include "src/net/shared/packet_queue.h"

static Client client = {0};

int client_thread(void* arg) {
	ENetHost* host = arg;
	ENetEvent event;

	while (SDL_GetAtomicInt(&client.connected)) {
		i32 rc = enet_host_service(host, &event, CLIENT_POLL_TIMEOUT_MS);
		if (rc < 0) {
			log_err("client enet_host_service error");
			break;
		}
		if (rc == 0) continue;
		do {
			switch (event.type) {
				case ENET_EVENT_TYPE_RECEIVE: {
					NetPacket* packet = packet_create_input((u8*)event.packet->data, (u32)event.packet->dataLength);
					if (packet) {
						queue_push(&client.queue, packet);
					}
					enet_packet_destroy(event.packet);
				}
				case ENET_EVENT_TYPE_CONNECT: {
					log_info("client: connect event on listener thread");
					SDL_SetAtomicInt(&client.connected, 1);
					break;
				}
				case ENET_EVENT_TYPE_DISCONNECT: {
					log_info("client: disconnected from server (type=%u, data=%u)", event.type, (u32)event.data);
					SDL_SetAtomicInt(&client.connected, false);
					break;
				}
				case ENET_EVENT_TYPE_NONE: {
					break;
				}
			}
		} while (SDL_GetAtomicInt(&client.connected) && enet_host_service(host, &event, 0) > 0);
	}

	log_info("disconnected from server");

	return 0;
}

b8 client_connect(ClientCreateInfo info) {
	if (SDL_GetAtomicInt(&client.connected)) {
		log_warn("client already connected to a network");
		return false;
	}

	queue_init(&client.queue);

	// create and set network managment
	client.net_manager = enet_host_create(
		NULL,
		1,
		CLIENT_MAX_CHANNELS,
		0,
		0
	);
	if (client.net_manager == NULL) {
		log_err("client failed to initialize with enet");
		return false;
	}
	if (enet_address_set_host(&client.address, info.server_address) != 0) {
		log_err("client could not resolve hostname '%d'", info.server_address.host);
		enet_host_destroy(client.net_manager);
		client.net_manager = NULL;
		queue_clear(&client.queue);
		return false;
	}
	client.address.port = info.server_address.port;
	client.address.host = info.server_address.host;

	// connecting to server
	client.server = enet_host_connect(client.net_manager, &client.address, CLIENT_MAX_CHANNELS, 0);
	if (!client.server) {
		log_err("enet_host_connect failed");
		enet_host_destroy(client.net_manager);
		client.net_manager = NULL;
		queue_clear(&client.queue);
		return false;
	}

	// wait for enet handshake between client and server
	ENetEvent event;
	b8 handshake = false;
	if (enet_host_service(client.net_manager, &event, CLIENT_POLL_TIMEOUT_MS) > 0 && event.type == ENET_EVENT_TYPE_CONNECT) {
		handshake = true;
		log_info("client succefully connected to %d:%d", info.server_address.host, info.server_address.port);
	} else {
		log_err("handshake with server timed out connecting to %d:%d", info.server_address.host, info.server_address.port);
		enet_peer_reset(client.server);
		client.server = NULL;
		enet_host_destroy(client.net_manager);
		client.host = NULL;
		queue_clear(&client.queue);
		return false;
	}

	SDL_SetAtomicInt(&client.connected, true);

	client.thread = SDL_CreateThread(client_thread, "clientthread", client.net_manager);
	if (!client.thread) {
		log_err("client: failed to create listener thread: %s", SDL_GetError());
		client_disconnect();
		return false;
	}

	return true;
}

b8 client_disconnect() {
	if (!SDL_GetAtomicInt(&client.connected)) {
		log_warn("client not connected to a server");
		return false;
	}

	enet_peer_disconnect(client.server, 0);


	// gives server time to acknowledge disconnection
	ENetEvent event;
	b8 clean_disconnect = false;
	u64 deadline = SDL_GetTicks() + 100;
	while (SDL_GetTicks() < deadline) {
		u32 ret = enet_host_connect(client.server, &event, deadline - SDL_GetTicks());
		if (ret <= 0) break;
		if (event.type == ENET_EVENT_TYPE_RECEIVE) {
			enet_packet_destroy(event.packet);
		} else if (event.type == ENET_EVENT_TYPE_DISCONNECT) {
			clean_disconnect = true;
			break;
		}
	}

	if (!clean_disconnect) {
		log_warn("client disconnect wasn't graceful, forcing reset");
		enet_peer_reset(client.server);
	}

	SDL_SetAtomicInt(&client.connected, false);
	client.server = NULL;
	queue_clear(&client.queue);
	return true;
}

void client_send_packet(NetPacket* packet) {
	if (!SDL_GetAtomicInt(&client.connected)) {
		log_warn("client not connect to server, recieving no packets");
		return;
	}

	u32 flags = ENET_PACKET_FLAG_RELIABLE;

	ENetPacket* packet = enet_packet_create(packet->data, packet->data_len, flags);
	if (!packet) {
		log_warn("enet failed to generate packet");
		return;
	}

	if (enet_peer_send(client.server, 0, packet) < 0) {
		log_err("failed to send packet");
		enet_packet_destroy(packet);
		return;
	}
	enet_host_flush(client.server);
}


NetPacket* client_poll_packet(void) {
	return queue_pop(&client.queue);
}


