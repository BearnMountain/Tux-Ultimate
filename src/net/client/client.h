#ifndef CLIENT_H_
#define CLIENT_H_

#include <enet/enet.h>
#include "src/util/defines.h"
#include "src/net/shared/packet.h"

typedef struct {
	ENetAddress server_address;
	u32 server_port;
	char* player_name;
} ClientCreateInfo;

b8 client_create(ClientCreateInfo info);
b8 client_destroy(void);

void client_send_packet(NetPacket* packet);

#endif
