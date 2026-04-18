#ifndef PACKET_H_
#define PACKET_H_

#include "src/util/defines.h"

typedef enum {
	PKT_INVALID,
	PKT_CONNECT,
	PKT_DISCONNECT,
	PKT_INPUT,
	PKT_STATE,
	PKT_CHAT
} PacketType;

// stored as *data, uncompress func will convert to type 
// and return that struct
typedef struct {
	PacketType type;
	u32 tick;
	char* data;
	u32 data_len; // will be compressed
} NetPacket;

// data is copied over, input arr can be discarded
NetPacket* packet_create_input(char* input_arr, u32 dat_len);

#endif
