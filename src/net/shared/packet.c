#include "packet.h"
#include "src/util/config.h"
#include "src/util/logger.h"
#include <stdlib.h>

ENetPacket* packet_serialize(NetPacket* packet, b8 cleanup) {
	ENetPacket* data = enet_packet_create(packet->data, packet->data_len, ENET_PACKET_FLAG_RELIABLE);
	packet_destroy(packet);
	return data;
}

NetPacket* packet_deserialize(ENetPacket* packet, b8 cleanup) {
	NetPacket* data = (NetPacket*)malloc(sizeof(NetPacket));
	data->data = (char*)malloc(packet->dataLength * sizeof(char));
	data->data_len = packet->dataLength;
	memcpy(data->data, packet->data, data->data_len * sizeof(char));
	data->type = PKT_CHAT;

	enet_packet_destroy(packet);
	return data;
}



ENetPacket* packet_create_enet(NetPacket* packet, b8 cleanup) {
	ENetPacket* epacket;

	switch (packet->type) {
		case PKT_INPUT:
			epacket = enet_packet_create(packet->data, packet->data_len, ENET_PACKET_FLAG_RELIABLE);
			break;
		case PKT_CONNECT:
			epacket = enet_packet_create(packet->data, packet->data_len, ENET_PACKET_FLAG_RELIABLE);
			log_warn("connect not implemented");
			break;
		case PKT_DISCONNECT:
			epacket = enet_packet_create(packet->data, packet->data_len, ENET_PACKET_FLAG_RELIABLE);
			log_warn("connect not implemented");
			break;
		case PKT_STATE:
			epacket = enet_packet_create(packet->data, packet->data_len, ENET_PACKET_FLAG_RELIABLE);
			log_warn("connect not implemented");
			break;
		case PKT_CHAT:
			epacket = enet_packet_create(packet->data, packet->data_len, ENET_PACKET_FLAG_RELIABLE);
			log_warn("connect not implemented");
			break;
		case PKT_INVALID:
			epacket = NULL;
			log_warn("invalid state");
			break;
	}

	if (cleanup) packet_destroy(packet);

	return epacket;
}

NetPacket* packet_create_input(char* input_arr, u32 data_len) {
	NetPacket* packet = (NetPacket*)malloc(sizeof(NetPacket));

	packet->type = PKT_INPUT;
	packet->tick = config.game_tick;
	packet->data = (char*)malloc(data_len * sizeof(char));
	memcpy(packet->data, input_arr, data_len);
	packet->data_len = data_len;

	return packet;
}


void packet_destroy(NetPacket* packet) {
	free(packet->data);
	free(packet);
}
