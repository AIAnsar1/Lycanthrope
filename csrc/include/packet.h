#pragma once


#ifndef LYCANTHROPE_PACKET_H
#define LYCANTHROPE_PACKET_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

#if defined(_MSC_VER)
    #define LYC_PACKED_BEGIN  __pragma(pack(push, 1))
    #define LYC_PACKED_END    __pragma(pack(pop))
    #define LYC_PACKED_ATTR
#else
    #define LYC_PACKED_BEGIN
    #define LYC_PACKED_END
    #define LYC_PACKED_ATTR   __attribute__((packed))
#endif

#define LYC_TH_FIN  0x01
#define LYC_TH_SYN  0x02
#define LYC_TH_RST  0x04
#define LYC_TH_PSH  0x08
#define LYC_TH_ACK  0x10
#define LYC_TH_URG  0x20

LYC_PACKED_BEGIN
typedef struct {
    uint32_t src_ip;
    uint32_t dst_ip;
    uint16_t src_port;
    uint16_t dst_port;
    uint32_t seq;
    uint32_t ack;
    uint8_t  flags;
    uint16_t window;
    uint8_t  ttl;
} LYC_PACKED_ATTR TcpPacketParams;
LYC_PACKED_END

int lyc_build_tcp_packet(const TcpPacketParams *params,const uint8_t *payload,size_t payload_len,uint8_t *out_buffer, size_t buffer_size);

#ifdef __cplusplus
}
#endif

#endif /* LYCANTHROPE_PACKET_H */


