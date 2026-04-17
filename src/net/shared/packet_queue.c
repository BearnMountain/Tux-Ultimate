#include "packet_queue.h"
#include <stdlib.h>

void queue_init(PacketQueue* q) {
    q->head = q->tail = NULL;
    pthread_mutex_init(&q->mutex, NULL);
}

void queue_push(PacketQueue* q, NetPacket* packet) {
    PacketNode* n = malloc(sizeof(PacketNode));
    n->packet = packet;
    n->next = NULL;

    pthread_mutex_lock(&q->mutex);
    if (q->tail)
        q->tail->next = n;
    else
        q->head = n;
    q->tail = n;
    pthread_mutex_unlock(&q->mutex);
}

NetPacket *queue_pop(PacketQueue* q) {
    pthread_mutex_lock(&q->mutex);
    PacketNode* n = q->head;
    if (!n) {
        pthread_mutex_unlock(&q->mutex);
        return NULL;
    }
    q->head = n->next;
    if (!q->head)
        q->tail = NULL;
    pthread_mutex_unlock(&q->mutex);

    NetPacket* p = n->packet;
    free(n);
    return p;
}
