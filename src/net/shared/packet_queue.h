#ifndef PACKET_QUEUE_H_
#define PACKET_QUEUE_H_

#include <pthread.h>
#include "packet.h"

typedef struct PacketNode {
    NetPacket* packet;
    struct PacketNode* next;
} PacketNode;

typedef struct PacketQueue {
    PacketNode* head;
    PacketNode* tail;
    pthread_mutex_t mutex;
} PacketQueue;

// static allocation, you create dynamic version
void queue_init(PacketQueue* q);
void queue_push(PacketQueue* q, NetPacket* packet);
NetPacket* queue_pop(PacketQueue* q);
void queue_clear(PacketQueue* q);

#endif
