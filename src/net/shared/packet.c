#include "packet.h"
#include "src/util/config.h"
#include <stdlib.h>

NetPacket* packet_create_input(char* input_arr, u32 data_len) {
	NetPacket* packet = (NetPacket*)malloc(sizeof(NetPacket));

	packet->type = PKT_INPUT;
	packet->tick = config.game_tick;
	memcpy(packet->data, input_arr, data_len);
	packet->data_len = data_len;

	return packet;
}
